//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1103/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1103<F: Float>(t4798: F, t9687: F, t415: F, t5204: F, t717: F, t5060: F, t705: F, t5064: F, t4830: F, t9656: F, t1333: F, t9688: F, t1693: F, t1763: F, t1772: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32974 = t9687 * t4798;
    let t32975 = t415 * t32974;
    let t32977 = t717 * t5204;
    let t32978 = t415 * t32977;
    let t32980 = t705 * t5060;
    let t32981 = t32980 * t5064;
    let t32982 = t415 * t32981;
    let t32984 = t4830 * t9656;
    let t32987 = t1333 * t9688;
    let t32989 = t1693 * t1763;
    let t32990 = t32989 * t1772;
    (t32974, t32975, t32977, t32978, t32980, t32981, t32982, t32984, t32987, t32989, t32990)
}
