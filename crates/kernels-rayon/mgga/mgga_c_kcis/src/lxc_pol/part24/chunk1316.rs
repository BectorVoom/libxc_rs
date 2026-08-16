//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1316/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1316(t1820: f64, t95286: f64, t101641: f64, t101661: f64, t101681: f64, t101701: f64, t101713: f64, t101716: f64, t101718: f64, t101720: f64, t101723: f64, t101730: f64, t101732: f64, t11223: f64, t11230: f64, t1282: f64, t1291: f64, t20721: f64, t27100: f64, t28260: f64, t29084: f64, t29087: f64, t47700: f64, t5363: f64, t6860: f64, t6879: f64, t7823: f64, t92398: f64, t92576: f64, t96670: f64) -> (f64, f64) {
    let t101734 = 2.0_f64 * t95286 * t1820;
    let t101735 = -6.0_f64 * t11230 * t29084 * t1291 - 6.0_f64 * t92398 * t20721 - 12.0_f64 * t47700 * t28260 - t1282 * (t101641 + t101661 + t101681 + t101701) - 6.0_f64 * t11230 * t7823 * t6860 + 4.0_f64 * t96670 * t5363 + 2.0_f64 * t11223 * t29084 - t101713 + 2.0_f64 * t92576 * t6860 + t101716 + t101718 + t101720 + t101723 - t27100 * t6879 - 12.0_f64 * t11230 * t29087 * t1291 - t101730 - t101732 + t101734;
    (t101734, t101735)
}
