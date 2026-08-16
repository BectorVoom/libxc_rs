//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2075/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2075<F: Float>(t10383: F, t964: F, t10868: F, t820: F, t1015: F, t10472: F, t42559: F, t204: F, t376: F, t1020: F, t1023: F, t248: F) -> (F, F, F, F, F) {
    let t43157 = t964 * t10383;
    let t43198 = t820 * t10868;
    let t43211 = t10472 * t1015 * t42559;
    let t43216 = t204 * t376;
    let t43219 = t1020 * t248 * t43216 * t1023;
    (t43157, t43198, t43211, t43216, t43219)
}
