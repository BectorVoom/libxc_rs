//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1324/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1324<F: Float>(t116676: F, t33056: F, t32995: F, t34073: F, t32909: F, t34154: F, t34090: F, t4811: F, t3805: F, t9949: F, t1333: F, t34057: F, t34167: F, t34270: F, t9660: F, t32889: F, t9918: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t116677 = t33056 * t116676;
    let t116701 = 0.69444444444444444446e-2 * t34073 * t32995;
    let t116703 = 0.69444444444444444446e-2 * t34073 * t32909;
    let t116705 = 0.26805555555555555556e-2 * t34154 * t32909;
    let t116723 = t4811 * t34090;
    let t116731 = t3805 * t9949;
    let t116736 = t1333 * t34057;
    let t116737 = 0.33163888888888888888e-2 * t116736;
    let t116738 = t1333 * t34167;
    let t116741 = 0.18518518518518518519e-1 * t34270 * t9660;
    let t116745 = t9918 * t32889;
    (t116677, t116701, t116703, t116705, t116723, t116731, t116736, t116737, t116738, t116741, t116745)
}
