//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1011/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1011<F: Float>(t17891: F, t7429: F, t16660: F, t5322: F, t5321: F, t17841: F, t17844: F, t17848: F, t17850: F, t17853: F, t17857: F, t17859: F, t17863: F, t17866: F, t17869: F, t17872: F, t17876: F, t17879: F, t17883: F, t17886: F, t17889: F) -> (F, F, F, F) {
    let t17892 = t7429 * t17891;
    let t17894 = t5322 * t16660;
    let t17895 = t5321 * t17894;
    let t17897 = -t17841 / 9.0 - t17844 / 128.0 + 3.0 / 128.0 * t17848 + t17850 / 12.0 - t17853 / 64.0 + t17857 / 6.0 - t17859 / 192.0 - t17863 / 8.0 + t17866 / 6.0 + t17869 / 4.0 - t17872 / 64.0 - 3.0 / 8.0 * t17876 - t17879 / 72.0 - t17883 / 16.0 - t17886 / 24.0 - t17889 / 36.0 - t17892 / 48.0 + t17895 / 96.0;
    (t17892, t17894, t17895, t17897)
}
