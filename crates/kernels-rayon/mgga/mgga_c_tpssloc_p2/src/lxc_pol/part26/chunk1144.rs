//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1144/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1144(t776: f64, t857: f64, t865: f64, t23270: f64, t22986: f64, t225: f64, t6625: f64, t6576: f64, t10049: f64, t1912: f64, t23236: f64, t23239: f64, t23243: f64, t23250: f64, t23252: f64, t23254: f64, t23259: f64, t23262: f64, t23266: f64, t2597: f64, t2720: f64, t2743: f64, t6627: f64, t6663: f64, t866: f64, t9590: f64, t9593: f64) -> (f64, f64, f64, f64, f64) {
    let t23272 = t857 * t776 * t865;
    let t23273 = t23270 * t23272;
    let t23274 = t22986 * t23273;
    let t23278 = t6625 * t225;
    let t23281 = t6576 * t225;
    let t23284 = t23236 - 0.3289868133696452873e-1_f64 * t23239 + 0.49348022005446793095e-1_f64 * t23243 - t9590 * t1912 - 2.0_f64 * t2597 * t6663 - t6627 * t2743 - t23250 + t23252 - 0.82246703342411321824e-2_f64 * t23254 + 0.82246703342411321825e-2_f64 * t23259 + t23262 - 2.0_f64 * t9593 * t1912 - 0.16449340668482264365e-1_f64 * t23266 - t10049 * t1912 + 0.3289868133696452873e-1_f64 * t23274 + 2.0_f64 * t6627 * t2720 - 2.0_f64 * t23278 * t866 - 2.0_f64 * t23281 * t866;
    (t23272, t23273, t23278, t23281, t23284)
}
