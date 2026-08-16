//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2145/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2145(t23053: f64, t4236: f64, t13173: f64, t6614: f64, t23041: f64, t13186: f64, t6621: f64, t81770: f64, t81772: f64, t81785: f64, t87222: f64, t87224: f64, t87226: f64, t87234: f64, t87235: f64, t87237: f64, t87241: f64, t87243: f64, t87245: f64, t87248: f64, t87249: f64) -> f64 {
    let t87251 = t23053 * t4236;
    let t87253 = t6614 * t13173;
    let t87255 = t23041 * t4236;
    let t87256 = 7.0_f64 / 1152.0_f64 * t87255;
    let t87257 = t6621 * t13186;
    let t87259 = -t87222 / 384.0_f64 - t87224 / 192.0_f64 - t87226 / 384.0_f64 - t87234 + 5.0_f64 / 384.0_f64 * t87235 - t87237 + 7.0_f64 / 288.0_f64 * t81770 + 7.0_f64 / 576.0_f64 * t81772 - 0.40372756094140390854e-3_f64 * t81785 + 5.0_f64 / 192.0_f64 * t87241 - 119.0_f64 / 6912.0_f64 * t87243 - t87245 / 1536.0_f64 + t87248 - t87249 / 1536.0_f64 - t87251 / 768.0_f64 - t87253 / 1536.0_f64 + t87256 - 5.0_f64 / 64.0_f64 * t87257;
    t87259
}
