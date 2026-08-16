//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3926/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3926(t116: f64, t21813: f64, t118: f64, t1310: f64, t1315: f64, t13544: f64, t1453: f64, t18235: f64, t18245: f64, t21881: f64, t22506: f64, t22525: f64, t2322: f64, t2328: f64, t2331: f64, t2371: f64, t27123: f64, t3813: f64, t4151: f64, t4254: f64, t4292: f64, t4293: f64, t511: f64, t5517: f64, t5528: f64, t5787: f64, t5884: f64, t651: f64, t671: f64, t6765: f64, t6773: f64, t68231: f64, t73306: f64, t73326: f64, t73343: f64, t73359: f64, t73376: f64, t73383: f64, t73400: f64, t73417: f64, t73495: f64, t73528: f64, t75357: f64, t75372: f64, t75386: f64, t75401: f64, t75408: f64, t75412: f64, t75421: f64, t7732: f64) -> (f64, f64) {
    let t75439 = t21813 * t116;
    let t75451 = -2.0_f64 * t5884 * t3813 - 2.0_f64 * t2328 * t6765 - t118 * (t68231 + t73306) + 2.0_f64 * t22525 * t1453 + t6773 * t4151 + 2.0_f64 * t1315 * t22506 + t511 * (t73326 + t73343 + t73359 + t73376 + t73383 + t73400 + t73417 + t73495 + t73528 + t75357 + t75372 + t75386 + t75401 + t75408 + t75412 + t75421) + 4.0_f64 * t5528 * t5787 - 8.0_f64 * t2322 * t18235 - 8.0_f64 * t4254 * t18235 - 8.0_f64 * t651 * t5517 * t4292 - 4.0_f64 * t651 * t1310 * t21881 - 4.0_f64 * t75439 * t671 - 4.0_f64 * t18245 * t2331 - 2.0_f64 * t651 * t6765 * t2371 - 8.0_f64 * t27123 * t4293 - 4.0_f64 * t7732 * t13544;
    (t75439, t75451)
}
