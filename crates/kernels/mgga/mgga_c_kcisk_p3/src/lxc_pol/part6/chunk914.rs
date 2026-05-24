//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 914/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk914<F: Float>(t1725: F, t29195: F, t2418: F, t8729: F, t23528: F, t2417: F, t10925: F, t10983: F, t1706: F, t17520: F, t17567: F, t23496: F, t29196: F, t29228: F, t29231: F, t29244: F, t45: F, t4858: F, t4909: F, t634: F, t7091: F, t8698: F, t8730: F, t8733: F) -> F {
    let t29250 = t29195 * t1725;
    let t29253 = t2418 * t8729;
    let t29256 = t23528 * t2417;
    let t29259 = F::new(3.0) * t23496 * t2418 + F::new(3.0) * t7091 * t8730 + F::cast_from(0.48245472966453314466e2_f64) * t17567 * t8733 - F::cast_from(0.96490945932906628932e2_f64) * t10983 * t29196 + F::new(1.0) * t1706 * t29228 + F::cast_from(0.51725014705706168417e3_f64) * t10925 * t29231 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t29244 * t634 - F::new(6.0) * t17520 * t8698 + F::new(6.0) * t4909 * t29250 - F::new(6.0) * t4858 * t29253 + F::cast_from(0.48245472966453314466e2_f64) * t4909 * t29256;
    t29259
}
