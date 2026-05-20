//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1953/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1953<F: Float>(t5: F, t101805: F, t101824: F, t101849: F, t101875: F, t101896: F, t101919: F, t101949: F, t101975: F, t117: F, t7535: F, t9593: F, t101767: F, t1310: F, t13425: F, t13532: F, t13540: F, t13544: F, t1843: F, t2056: F, t2089: F, t2322: F, t26154: F, t26399: F, t26676: F, t27123: F, t28196: F, t28198: F, t28652: F, t28658: F, t28696: F, t4246: F, t4248: F, t4254: F, t4293: F, t508: F, t5517: F, t651: F, t7359: F, t7367: F, t7373: F, t7474: F, t98484: F, t98487: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t101979 = piecewise3::<F>(t8, F::new(0.0), t101805 + t101824 + t101849 + t101875 + t101896 + t101919 + t101949 + t101975);
    let t101980 = t101979 * t117;
    let t102005 = t7535 * t9593;
    let t102009 = -F::new(2.0) * t101767 * t508 - t13425 * t2089 - F::new(2.0) * t4246 * t7474 - F::new(4.0) * t7359 * t13540 - F::new(4.0) * t7359 * t13532 - F::new(2.0) * t26676 * t1843 - t101980 * t508 - F::new(2.0) * t28652 * t1310 - F::new(2.0) * t4248 * t26154 - F::new(2.0) * t98484 * t2056 - F::new(4.0) * t98487 * t2056 - F::new(4.0) * t27123 * t7367 - F::new(4.0) * t2322 * t28696 - F::new(4.0) * t4254 * t28696 - F::new(4.0) * t651 * t5517 * t7373 - F::new(4.0) * t26399 * t4293 - F::new(4.0) * t28658 * t4293 - F::new(2.0) * t7359 * t13544 + F::new(4.0) * t28196 * t102005 * t28198;
    (t101980, t102009)
}
