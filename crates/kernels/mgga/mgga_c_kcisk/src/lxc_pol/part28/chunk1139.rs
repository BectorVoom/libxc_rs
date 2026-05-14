//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1139/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1139<F: Float>(t4998: F, t9666: F, t9664: F, t1863: F, t1871: F, t5060: F, t705: F, t1333: F, t9688: F, t1693: F, t1763: F, t1772: F) -> (F, F, F, F, F, F, F) {
    let t32955 = t4998 * t9666;
    let t32956 = t9664 * t32955;
    let t32965 = t1863 * t1871;
    let t32980 = t705 * t5060;
    let t32987 = t1333 * t9688;
    let t32989 = t1693 * t1763;
    let t32990 = t32989 * t1772;
    (t32955, t32956, t32965, t32980, t32987, t32989, t32990)
}
