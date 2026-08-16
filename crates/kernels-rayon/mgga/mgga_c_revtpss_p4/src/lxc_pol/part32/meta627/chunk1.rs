//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2003/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2003(t5: f64, t109895: f64, t109918: f64, t109945: f64, t109970: f64, t109992: f64, t110012: f64, t110027: f64, t110049: f64, t117: f64, t108126: f64, t109263: f64, t109368: f64, t109399: f64, t109423: f64, t109446: f64, t109467: f64, t109493: f64, t109516: f64, t109533: f64, t109563: f64, t109598: f64, t109628: f64, t109656: f64, t109681: f64, t109704: f64, t109724: f64, t109756: f64, t109864: f64, t109874: f64, t1310: f64, t13426: f64, t1450: f64, t18227: f64, t2014: f64, t2089: f64, t21881: f64, t21882: f64, t25082: f64, t26405: f64, t28196: f64, t28286: f64, t28727: f64, t28750: f64, t28935: f64, t29498: f64, t30209: f64, t30511: f64, t30553: f64, t34251: f64, t4248: f64, t4254: f64, t4293: f64, t508: f64, t532: f64, t5517: f64, t649: f64, t651: f64, t7359: f64, t7898: f64, t7983: f64, t7988: f64, t95472: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t110053 = piecewise3(t8, 0.0_f64, t109895 + t109918 + t109945 + t109970 + t109992 + t110012 + t110027 + t110049);
    let t110054 = t110053 * t117;
    let t110058 = -4.0_f64 * t4254 * t30209 - 4.0_f64 * t651 * t5517 * t7983 - 3.0_f64 * t25082 * t26405 * t108126 - 2.0_f64 * t651 * t508 * t109368 - 4.0_f64 * t13426 * t7988 - 4.0_f64 * t18227 * t7988 - 4.0_f64 * t4248 * t28750 + 6.0_f64 * t2014 * t95472 * t29498 + 6.0_f64 * t7898 * t28935 - 2.0_f64 * t651 * t2089 * t21881 - 2.0_f64 * t7359 * t21882 - 4.0_f64 * t34251 * t4293 + t2014 * t532 * (t109399 + t109423 + t109446 + t109467 + t109493 + t109516 + t109533 + t109563 + t109598 + t109628 + t109656 + t109681 + t109704 + t109724 + t109756 + t109864) * t1450 - 2.0_f64 * t7898 * t28727 + 6.0_f64 * t2014 * t109874 * t29498 + 2.0_f64 * t28196 * t28286 * t109263 - t110054 * t508 - t30553 * t1310 - t649 * t30511;
    (t110054, t110058)
}
