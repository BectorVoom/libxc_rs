//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1165/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1165<F: Float>(t6638: F, t92564: F, t19826: F, t7766: F, t19836: F, t92581: F, t29036: F, t33853: F, t10498: F, t1203: F, t33862: F, t5039: F, t96543: F, t1820: F, t95286: F, t101641: F, t101661: F, t101681: F, t101701: F, t11223: F, t11230: F, t1282: F, t1291: F, t20721: F, t27100: F, t28260: F, t29084: F, t29087: F, t47700: F, t5363: F, t6860: F, t6879: F, t7823: F, t92398: F, t92576: F, t96670: F) -> (F, F, F, F, F, F, F, F, F) {
    let t101713 = 2.0 * t92564 * t6638;
    let t101716 = t19826 * t7766;
    let t101718 = 6.0 * t92581 * t19836;
    let t101720 = 6.0 * t33853 * t29036;
    let t101723 = 6.0 * t10498 * t7766 * t6638;
    let t101730 = 24.0 * t33862 * t29036 * t1203;
    let t101732 = 4.0 * t96543 * t5039;
    let t101734 = 2.0 * t95286 * t1820;
    let t101735 = -6.0 * t11230 * t29084 * t1291 - 6.0 * t92398 * t20721 - 12.0 * t47700 * t28260 - t1282 * (t101641 + t101661 + t101681 + t101701) - 6.0 * t11230 * t7823 * t6860 + 4.0 * t96670 * t5363 + 2.0 * t11223 * t29084 - t101713 + 2.0 * t92576 * t6860 + t101716 + t101718 + t101720 + t101723 - t27100 * t6879 - 12.0 * t11230 * t29087 * t1291 - t101730 - t101732 + t101734;
    (t101713, t101716, t101718, t101720, t101723, t101730, t101732, t101734, t101735)
}
