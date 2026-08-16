//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1166/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1166(t1882: f64, t36220: f64, t36165: f64, t8392: f64, t55797: f64, t7672: f64, t10443: f64, t10683: f64, t10696: f64, t10703: f64, t1091: f64, t112663: f64, t112760: f64, t113656: f64, t114847: f64, t11593: f64, t1255: f64, t143621: f64, t143998: f64, t144262: f64, t152772: f64, t15299: f64, t15312: f64, t15369: f64, t15460: f64, t1901: f64, t2749: f64, t2874: f64, t29071: f64, t29128: f64, t29198: f64, t29203: f64, t29208: f64, t296: f64, t33830: f64, t34197: f64, t34202: f64, t36060: f64, t36112: f64, t36126: f64, t36186: f64, t3746: f64, t4162: f64, t4167: f64, t4181: f64, t446: f64, t6361: f64, t684: f64, t7679: f64, t840: f64) -> (f64, f64) {
    let t154399 = t1882 * t36220;
    let t154439 = t8392 * t36165;
    let t154463 = t55797 * t7672;
    let t154467 = -2.0_f64 / 9.0_f64 * t154399 - 2.0_f64 * t1901 * t29128 * t10696 * t7679 * t4181 + t1901 * t10443 * t36126 / 9.0_f64 + t1901 * t2874 * t144262 * t1091 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t11593 * t2874 * t34202 * t3746 - 2.0_f64 * t446 * t10683 * t1255 * t33830 - 2.0_f64 / 9.0_f64 * t1901 * t15299 * t152772 + 2.0_f64 / 3.0_f64 * t446 * t840 * t2749 * t36112 + 2.0_f64 / 9.0_f64 * t1901 * t113656 * t6361 + 4.0_f64 / 3.0_f64 * t1901 * t15369 * t34197 * t4162 + 2.0_f64 * t1901 * t15460 * t143621 * t4167 + 2.0_f64 / 27.0_f64 * t154439 - 2.0_f64 * t1901 * t29071 * t34202 * t4162 - 4.0_f64 / 9.0_f64 * t1901 * t112760 * t29198 - 4.0_f64 / 9.0_f64 * t1901 * t114847 * t29203 + 4.0_f64 / 27.0_f64 * t1901 * t112663 * t29208 - t1901 * t10703 * t36186 * t684 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t15312 * t36060 * t684 - 2.0_f64 / 9.0_f64 * t143998 + 2.0_f64 / 3.0_f64 * t446 * t296 * t154463;
    (t154463, t154467)
}
