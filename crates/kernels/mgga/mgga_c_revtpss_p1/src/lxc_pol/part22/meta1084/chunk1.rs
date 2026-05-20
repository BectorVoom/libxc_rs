//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3926/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3926<F: Float>(t116: F, t21813: F, t118: F, t1310: F, t1315: F, t13544: F, t1453: F, t18235: F, t18245: F, t21881: F, t22506: F, t22525: F, t2322: F, t2328: F, t2331: F, t2371: F, t27123: F, t3813: F, t4151: F, t4254: F, t4292: F, t4293: F, t511: F, t5517: F, t5528: F, t5787: F, t5884: F, t651: F, t671: F, t6765: F, t6773: F, t68231: F, t73306: F, t73326: F, t73343: F, t73359: F, t73376: F, t73383: F, t73400: F, t73417: F, t73495: F, t73528: F, t75357: F, t75372: F, t75386: F, t75401: F, t75408: F, t75412: F, t75421: F, t7732: F) -> (F, F) {
    let t75439 = t21813 * t116;
    let t75451 = -F::new(2.0) * t5884 * t3813 - F::new(2.0) * t2328 * t6765 - t118 * (t68231 + t73306) + F::new(2.0) * t22525 * t1453 + t6773 * t4151 + F::new(2.0) * t1315 * t22506 + t511 * (t73326 + t73343 + t73359 + t73376 + t73383 + t73400 + t73417 + t73495 + t73528 + t75357 + t75372 + t75386 + t75401 + t75408 + t75412 + t75421) + F::new(4.0) * t5528 * t5787 - F::new(8.0) * t2322 * t18235 - F::new(8.0) * t4254 * t18235 - F::new(8.0) * t651 * t5517 * t4292 - F::new(4.0) * t651 * t1310 * t21881 - F::new(4.0) * t75439 * t671 - F::new(4.0) * t18245 * t2331 - F::new(2.0) * t651 * t6765 * t2371 - F::new(8.0) * t27123 * t4293 - F::new(4.0) * t7732 * t13544;
    (t75439, t75451)
}
