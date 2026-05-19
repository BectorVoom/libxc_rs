//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1194/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1194<F: Float>(t1025: F, t2630: F, t7255: F, t7485: F, t7497: F, t1112: F, t7345: F, t2662: F, t2676: F, t2640: F, t7491: F, t7554: F) -> (F, F, F, F, F, F, F) {
    let t22115 = F::cast_from(0.38527786510141256862e1_f64) * t2630 * t1025 * t7255;
    let t22116 = t7497 * t7485;
    let t22120 = F::cast_from(0.67471172535210825684e-1_f64) * t2630 * t7345 * t1112;
    let t22123 = F::cast_from(0.43374325201206959368e-1_f64) * t2630 * t2662 * t2676;
    let t22126 = F::cast_from(0.12842595503380418954e1_f64) * t2630 * t2662 * t2640;
    let t22127 = t7497 * t7491;
    let t22131 = F::cast_from(0.21687162600603479684e-1_f64) * t2630 * t1025 * t7554;
    (t22115, t22116, t22120, t22123, t22126, t22127, t22131)
}
