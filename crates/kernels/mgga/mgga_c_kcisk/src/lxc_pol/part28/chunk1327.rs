//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1327/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1327<F: Float>(t18325: F, t34153: F, t1333: F, t34242: F, t32942: F, t34097: F, t1763: F, t1772: F, t7278: F, t32889: F, t9927: F, t34060: F, t34164: F, t3805: F, t9952: F, t5030: F, t658: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t116965 = t34153 * t18325;
    let t116970 = t1333 * t34242;
    let t116971 = 0.88437037037037037034e-2 * t116970;
    let t116979 = 0.69444444444444444446e-2 * t32942 * t34097;
    let t116983 = t7278 * t1763 * t1772;
    let t116994 = t9927 * t32889;
    let t116996 = t1333 * t34060;
    let t116997 = 0.33163888888888888888e-2 * t116996;
    let t116998 = t1333 * t34164;
    let t116999 = 0.88437037037037037034e-2 * t116998;
    let t117008 = t3805 * t9952;
    let t117019 = t658 * t5030;
    (t116965, t116970, t116971, t116979, t116983, t116994, t116996, t116997, t116998, t116999, t117008, t117019)
}
