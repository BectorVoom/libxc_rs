//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1305/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1305<F: Float>(t34090: F, t4811: F, t112192: F, t33033: F, t7256: F, t3805: F, t9949: F, t1333: F, t34057: F, t34167: F, t34270: F, t9660: F, t32965: F, t415: F, t7070: F, t32889: F, t9918: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116723 = t4811 * t34090;
    let t116726 = t112192 * t7256 * t33033;
    let t116731 = t3805 * t9949;
    let t116736 = t1333 * t34057;
    let t116737 = 0.33163888888888888888e-2 * t116736;
    let t116738 = t1333 * t34167;
    let t116741 = 0.18518518518518518519e-1 * t34270 * t9660;
    let t116743 = t415 * t32965 * t7070;
    let t116745 = t9918 * t32889;
    (t116723, t116726, t116731, t116736, t116737, t116738, t116741, t116743, t116745)
}
