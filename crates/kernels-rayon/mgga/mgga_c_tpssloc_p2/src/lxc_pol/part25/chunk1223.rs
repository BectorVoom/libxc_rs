//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1223/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1223(t28: f64, t265: f64, t504: f64, t85243: f64, t2071: f64, t2250: f64, t24420: f64, t52: f64, t607: f64, t7150: f64, t85296: f64, t85337: f64, t9258: f64, t113: f64, t11968: f64, t12156: f64, t1390: f64, t15904: f64, t1983: f64, t2036: f64, t2075: f64, t2094: f64, t2095: f64, t22574: f64, t22596: f64, t22607: f64, t2312: f64, t23857: f64, t23958: f64, t24169: f64, t24428: f64, t24432: f64, t24433: f64, t26161: f64, t26558: f64, t32193: f64, t39367: f64, t510: f64, t55173: f64, t55246: f64, t650: f64, t652: f64, t671: f64, t6876: f64, t7156: f64, t7171: f64, t7217: f64, t7218: f64, t83695: f64, t83886: f64, t84149: f64, t84733: f64, t85254: f64, t9351: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t85339 = piecewise3(t505, 0.0_f64, t85243);
    let t85349 = piecewise3(t401, t85296 + t85337, t85339 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24420 * t607 - 3.0_f64 / 2.0_f64 * t7150 * t2250 - t2071 * t9258 / 2.0_f64);
    let t85370 = -6.0_f64 * t1983 * t2095 * t83695 - 9.0_f64 * t22574 * t24432 * t55246 + 9.0_f64 * t22607 * t7171 - 9.0_f64 * t22574 * t24432 * t39367 - 6.0_f64 * t84149 * t510 + 18.0_f64 * t1983 * t84733 * t22596 - 3.0_f64 * t650 * t24428 - 3.0_f64 * t2312 * t7156 - 6.0_f64 * t9351 * t2075 + 3.0_f64 * t22607 * t7218 - 18.0_f64 * t83886 * t24433 - 18.0_f64 * t22574 * t32193 * t15904 - t113 * (t85254 + t85349) + 6.0_f64 * t1983 * t12156 * t2094 * t1390 - t2036 * t11968 + 6.0_f64 * t6876 * t24169 - 6.0_f64 * t652 * t24428 * t671 + 6.0_f64 * t26161 * t26558 * t55173 + 6.0_f64 * t1983 * t7217 * t23857 + 18.0_f64 * t6876 * t23958;
    t85370
}
