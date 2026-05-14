//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1019/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1019<F: Float>(t3103: F, t3109: F, t46715: F, t3079: F, t5276: F, t1121: F, t3137: F, t5313: F, t2639: F, t5416: F, t2885: F, t5454: F, t1220: F, t5097: F, t7274: F, t2838: F, t490: F, t5440: F) -> (F, F, F, F, F, F, F) {
    let t47069 = t3103 * t46715 * t3109;
    let t47138 = t5276 * t3079;
    let t47149 = t1121 * t3137 * t5313;
    let t47155 = t5416 * t2639;
    let t47331 = t5454 * t2885;
    let t47639 = t1220 * t7274 * t5097;
    let t47654 = t490 * t5440 * t2838;
    (t47069, t47138, t47149, t47155, t47331, t47639, t47654)
}
