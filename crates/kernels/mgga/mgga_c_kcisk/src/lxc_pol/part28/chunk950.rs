//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 950/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk950<F: Float>(t16974: F, t22426: F, t2063: F, t2487: F, t11402: F, t1648: F, t4657: F, t695: F, t1824: F, t8504: F, t1060: F, t11269: F, t2372: F, t16826: F, t16804: F, t8536: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22431 = t16974 * t22426;
    let t22434 = t2063 * t2487;
    let t22436 = t11402 * t22434 * t1648;
    let t22439 = t4657 * t695;
    let t22440 = t22434 * t1824;
    let t22441 = t22439 * t22440;
    let t22445 = t8504 * t695;
    let t22447 = t11269 * t22445 * t1060;
    let t22450 = t2372 * t2487;
    let t22452 = t16826 * t22450 * t1648;
    let t22456 = t16804 * t22450 * t1824;
    let t22459 = t8536 * t695;
    (t22431, t22434, t22436, t22440, t22441, t22447, t22452, t22456, t22459)
}
