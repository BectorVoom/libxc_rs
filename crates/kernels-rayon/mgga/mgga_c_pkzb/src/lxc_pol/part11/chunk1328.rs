//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1328/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1328(t10135: f64, t1220: f64, t10131: f64, t11383: f64, t904: f64, t10258: f64, t10263: f64, t23318: f64, t23332: f64, t23338: f64, t23341: f64, t28227: f64, t28231: f64, t28234: f64, t28263: f64, t28266: f64) -> f64 {
    let t32169 = t1220 * t10135;
    let t32171 = t1220 * t10131;
    let t32177 = t11383 * t904;
    let t32183 = t32169 / 36.0_f64 - t32171 / 18.0_f64 + 0.51448821741683684367e-2_f64 * t28227 - 0.34299214494455789578e-2_f64 * t28231 + 0.17149607247227894789e-2_f64 * t28234 + t23318 - t23332 - 5.0_f64 / 432.0_f64 * t23338 - 77.0_f64 / 486.0_f64 * t32177 + t23341 - 0.17149607247227894789e-2_f64 * t28263 + 0.17149607247227894789e-2_f64 * t28266 + 0.82318114786693894988e-1_f64 * t10258 * t10263;
    t32183
}
