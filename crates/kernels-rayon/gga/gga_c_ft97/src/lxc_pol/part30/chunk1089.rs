//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1089/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1089(t36092: f64, t92: f64, t25462: f64, t36109: f64, t317: f64, t35972: f64, t1253: f64, t7611: f64, t10683: f64, t142941: f64, t1506: f64, t2347: f64, t2360: f64, t25412: f64, t2665: f64, t28496: f64, t28938: f64, t28944: f64, t28997: f64, t29008: f64, t29026: f64, t33808: f64, t33996: f64, t34008: f64, t3886: f64, t4162: f64, t4255: f64, t6216: f64, t6217: f64, t6219: f64, t684: f64) -> f64 {
    let t152547 = t36092 * t92;
    let t152558 = t25462 * t36109;
    let t152560 = t35972 * t317;
    let t152565 = t7611 * t1253;
    let t152574 = 2.0_f64 / 9.0_f64 * t6216 * t28938 * t1506 * t2360 * t3886 - 2.0_f64 / 27.0_f64 * t6216 * t28944 * t1506 * t2347 * t3886 + 2.0_f64 / 9.0_f64 * t6216 * t25412 * t142941 * t4255 - t33808 * t28997 / 18.0_f64 - t152547 * t6219 / 18.0_f64 + 2.0_f64 * t6216 * t10683 * t33996 * t4162 - t33808 * t29026 / 18.0_f64 + t29008 * t34008 / 9.0_f64 + t152558 / 27.0_f64 - t6216 * t2665 * t152560 * t684 / 18.0_f64 - t6216 * t2665 * t152565 * t684 / 18.0_f64 + 2.0_f64 * t6216 * t10683 * t6217 * t28496;
    t152574
}
