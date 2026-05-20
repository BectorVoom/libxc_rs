//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2003/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2003<F: Float>(t5: F, t109895: F, t109918: F, t109945: F, t109970: F, t109992: F, t110012: F, t110027: F, t110049: F, t117: F, t108126: F, t109263: F, t109368: F, t109399: F, t109423: F, t109446: F, t109467: F, t109493: F, t109516: F, t109533: F, t109563: F, t109598: F, t109628: F, t109656: F, t109681: F, t109704: F, t109724: F, t109756: F, t109864: F, t109874: F, t1310: F, t13426: F, t1450: F, t18227: F, t2014: F, t2089: F, t21881: F, t21882: F, t25082: F, t26405: F, t28196: F, t28286: F, t28727: F, t28750: F, t28935: F, t29498: F, t30209: F, t30511: F, t30553: F, t34251: F, t4248: F, t4254: F, t4293: F, t508: F, t532: F, t5517: F, t649: F, t651: F, t7359: F, t7898: F, t7983: F, t7988: F, t95472: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t110053 = piecewise3::<F>(t8, F::new(0.0), t109895 + t109918 + t109945 + t109970 + t109992 + t110012 + t110027 + t110049);
    let t110054 = t110053 * t117;
    let t110058 = -F::new(4.0) * t4254 * t30209 - F::new(4.0) * t651 * t5517 * t7983 - F::new(3.0) * t25082 * t26405 * t108126 - F::new(2.0) * t651 * t508 * t109368 - F::new(4.0) * t13426 * t7988 - F::new(4.0) * t18227 * t7988 - F::new(4.0) * t4248 * t28750 + F::new(6.0) * t2014 * t95472 * t29498 + F::new(6.0) * t7898 * t28935 - F::new(2.0) * t651 * t2089 * t21881 - F::new(2.0) * t7359 * t21882 - F::new(4.0) * t34251 * t4293 + t2014 * t532 * (t109399 + t109423 + t109446 + t109467 + t109493 + t109516 + t109533 + t109563 + t109598 + t109628 + t109656 + t109681 + t109704 + t109724 + t109756 + t109864) * t1450 - F::new(2.0) * t7898 * t28727 + F::new(6.0) * t2014 * t109874 * t29498 + F::new(2.0) * t28196 * t28286 * t109263 - t110054 * t508 - t30553 * t1310 - t649 * t30511;
    (t110054, t110058)
}
