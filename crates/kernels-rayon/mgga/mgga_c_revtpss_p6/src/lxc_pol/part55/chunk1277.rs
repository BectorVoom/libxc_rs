//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1277/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1277(t2126: f64, t4292: f64, t1493: f64, t68: f64, t640: f64, t119457: f64, t122886: f64, t122890: f64, t122893: f64, t122901: f64, t124193: f64, t124200: f64, t124210: f64, t124220: f64, t124238: f64, t125260: f64, t125268: f64, t125279: f64, t125336: f64, t129180: f64, t129232: f64, t129236: f64, t1469: f64, t32795: f64, t32802: f64, t32806: f64, t33265: f64, t33268: f64, t33270: f64, t33275: f64, t33277: f64, t34402: f64, t34765: f64, t34771: f64, t4186: f64, t4237: f64, t644: f64, t8442: f64, t8621: f64, t8737: f64, t8881: f64, t95334: f64) -> (f64, f64) {
    let t129470 = t2126 * t4292;
    let t130808 = t68 * t1493;
    let t130831 = t640 * t68;
    let t130845 = 5.0_f64 / 18.0_f64 * t122901 * t34765 + 5.0_f64 / 18.0_f64 * t122890 * t34765 + 5.0_f64 / 18.0_f64 * t32802 * t8442 * t95334 * t1469 + 5.0_f64 / 18.0_f64 * t32802 * t8442 * t33268 * t4186 - 5.0_f64 / 9.0_f64 * t129232 * t8621 * t8881 * t1469 + 5.0_f64 / 18.0_f64 * t129236 * t33270 + 5.0_f64 / 6.0_f64 * t122886 * t119457 * t130808 * t644 - 10.0_f64 / 9.0_f64 * t124210 + 10.0_f64 / 27.0_f64 * t124220 + 5.0_f64 / 9.0_f64 * t32802 * t124200 * t125279 - 5.0_f64 / 3.0_f64 * t122893 * t124193 * t125336 - 5.0_f64 / 3.0_f64 * t122893 * t124193 * t125260 + 5.0_f64 / 9.0_f64 * t32802 * t124200 * t125268 - 5.0_f64 / 36.0_f64 * t32795 * t34771 - 5.0_f64 / 36.0_f64 * t32806 * t34771 - 5.0_f64 / 36.0_f64 * t8737 * t8621 * t130831 * t1493 - 5.0_f64 / 36.0_f64 * t8737 * t8621 * t33275 * t4237 + 5.0_f64 / 12.0_f64 * t129180 * t33265 - 5.0_f64 / 36.0_f64 * t34402 * t33277 - 20.0_f64 / 27.0_f64 * t124238;
    (t129470, t130845)
}
