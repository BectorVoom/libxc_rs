//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2497/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2497(t21684: f64, t225: f64, t1066: f64, t14529: f64, t14555: f64, t1635: f64, t17575: f64, t18061: f64, t18062: f64, t18074: f64, t18166: f64, t21663: f64, t21692: f64, t25757: f64, t3169: f64, t388: f64, t4557: f64, t4657: f64, t4694: f64, t50628: f64, t5848: f64, t5944: f64, t61646: f64, t63215: f64) -> f64 {
    let t70987 = t21684 * t225;
    let t71015 = -18.0_f64 * t18061 * t25757 * t50628 + 3.0_f64 * t388 * t4657 * t5848 - 3.0_f64 * t1066 * t70987 - 3.0_f64 * t14529 * t5944 - 3.0_f64 * t14555 * t5944 - 3.0_f64 * t1635 * t61646 - 3.0_f64 * t1635 * t63215 - 3.0_f64 * t17575 * t4694 + 6.0_f64 * t18062 * t4557 - 3.0_f64 * t18074 * t4694 - 3.0_f64 * t18166 * t4557 - t21663 * t3169 + 6.0_f64 * t21692 * t3169;
    t71015
}
