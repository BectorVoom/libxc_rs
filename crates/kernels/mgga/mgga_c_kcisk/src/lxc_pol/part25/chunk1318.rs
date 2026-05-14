//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1318/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1318<F: Float>(t32889: F, t9927: F, t1333: F, t34060: F, t34164: F, t17137: F, t32980: F, t415: F, t15937: F, t717: F, t3805: F, t9952: F, t112586: F, t17040: F, t5182: F, t112571: F, t112574: F, t112576: F, t116188: F, t116544: F, t116929: F, t33056: F) -> (F, F, F, F, F, F, F) {
    let t116994 = t9927 * t32889;
    let t116996 = t1333 * t34060;
    let t116997 = 0.33163888888888888888e-2 * t116996;
    let t116998 = t1333 * t34164;
    let t116999 = 0.88437037037037037034e-2 * t116998;
    let t117001 = t415 * t32980 * t17137;
    let t117004 = t415 * t717 * t15937;
    let t117008 = t3805 * t9952;
    let t117016 = t5182 * t112586 * t17040;
    let t117018 = 0.13402777777777777778e-2 * t33056 * t116544 + 0.23148148148148148149e-2 * t116994 - t116997 + t116999 - 0.13265555555555555555e-1 * t117001 + 0.14739506172839506172e-2 * t117004 - 0.23148148148148148148e-2 * t112571 - 0.11574074074074074074e-2 * t112574 + 0.55273148148148148147e-3 * t117008 - 0.11054629629629629629e-2 * t112576 - 0.26805555555555555556e-2 * t33056 * t116929 - 0.53611111111111111112e-2 * t33056 * t116188 - 0.22109259259259259258e-2 * t117016;
    (t116996, t116998, t117001, t117004, t117008, t117016, t117018)
}
