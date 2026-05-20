//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3017/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3017<F: Float>(t40593: F, t4452: F, t10777: F, t14671: F, t14686: F, t2646: F, t4343: F, t836: F, t10943: F, t14931: F, t14933: F, t2482: F, t2668: F, t2719: F) -> (F, F, F, F, F) {
    let t50634 = t40593 * t4452;
    let t50643 = t10777 * t14686 * t14671 * t2646;
    let t50649 = t4343 * t836;
    let t50673 = t14931 * t14686 * t14671 * t10943;
    let t50681 = t2482 * t2719 * t2668 * t14933;
    (t50634, t50643, t50649, t50673, t50681)
}
