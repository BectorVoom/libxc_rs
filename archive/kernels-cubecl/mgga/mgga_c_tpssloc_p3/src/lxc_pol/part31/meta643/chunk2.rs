//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1913/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1913<F: Float>(t23270: F, t25038: F, t776: F, t98169: F, t1888: F, t2717: F, t5657: F, t865: F, t1880: F, t23237: F, t28294: F, t22986: F, t28267: F, t82159: F) -> (F, F, F, F) {
    let t98172 = t25038 * t23270 * t98169 * t776;
    let t98181 = t1888 * t23270 * t2717 * t5657 * t865;
    let t98189 = t1880 * t23237 * t28294;
    let t98192 = t22986 * t82159 * t28267;
    (t98172, t98181, t98189, t98192)
}
