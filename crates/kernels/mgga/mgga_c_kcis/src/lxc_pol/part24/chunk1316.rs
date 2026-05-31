//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1316/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1316<F: Float>(t1820: F, t95286: F, t101641: F, t101661: F, t101681: F, t101701: F, t101713: F, t101716: F, t101718: F, t101720: F, t101723: F, t101730: F, t101732: F, t11223: F, t11230: F, t1282: F, t1291: F, t20721: F, t27100: F, t28260: F, t29084: F, t29087: F, t47700: F, t5363: F, t6860: F, t6879: F, t7823: F, t92398: F, t92576: F, t96670: F) -> (F, F) {
    let t101734 = F::cast_from(2.0_f64) * t95286 * t1820;
    let t101735 = -F::cast_from(6.0_f64) * t11230 * t29084 * t1291 - F::cast_from(6.0_f64) * t92398 * t20721 - F::cast_from(12.0_f64) * t47700 * t28260 - t1282 * (t101641 + t101661 + t101681 + t101701) - F::cast_from(6.0_f64) * t11230 * t7823 * t6860 + F::cast_from(4.0_f64) * t96670 * t5363 + F::cast_from(2.0_f64) * t11223 * t29084 - t101713 + F::cast_from(2.0_f64) * t92576 * t6860 + t101716 + t101718 + t101720 + t101723 - t27100 * t6879 - F::cast_from(12.0_f64) * t11230 * t29087 * t1291 - t101730 - t101732 + t101734;
    (t101734, t101735)
}
