//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2296/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2296<F: Float>(t27526: F, t86094: F, t24660: F, t24850: F, t1409: F, t3507: F, t24667: F, t24847: F, t64825: F, t974: F, t8067: F, t85660: F) -> (F, F, F, F, F, F) {
    let t94947 = F::cast_from(0.18277045187202515961e-2_f64) * t86094 * t27526;
    let t94948 = t24660 * t24850;
    let t94949 = t1409 * t3507;
    let t94954 = t24667 * t24850;
    let t94963 = t24847 * t974 * t64825;
    let t94966 = t85660 * t8067;
    (t94947, t94948, t94949, t94954, t94963, t94966)
}
