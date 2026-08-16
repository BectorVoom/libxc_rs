//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2121/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2121<F: Float>(t86868: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t22986: F, t23272: F, t23270: F, t2379: F, t25038: F, t25053: F) -> (F, F, F, F) {
    let t86869 = F::cast_from(0.76763589786250567036e-1_f64) * t86868;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    let t86875 = t22986 * t86873 * t23272;
    let t86881 = t25038 * t23270 * t25053 * t2379;
    (t86869, t86870, t86875, t86881)
}
