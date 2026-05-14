//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 556/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk556<F: Float>(t675: F, t7682: F, t1990: F, t2191: F, t1274: F, t1986: F, t1173: F, t2189: F, t674: F, t1997: F, t1240: F, t128: F, t118: F, t1994: F, t1249: F, t687: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7683 = t675 * t7682;
    let t7684 = 0.12769379967989351819e-4 * t7683;
    let t7685 = t2191 * t1990;
    let t7686 = 0.85129199786595678796e-5 * t7685;
    let t7687 = t1986 * t1274;
    let t7688 = t675 * t7687;
    let t7689 = 0.42564599893297839398e-5 * t7688;
    let t7690 = t2189 * t1173;
    let t7691 = t7690 * t674;
    let t7692 = t7691 * t1997;
    let t7693 = 0.1064114997332445985e-4 * t7692;
    let t7694 = t128 * t1240;
    let t7695 = t118 * t7694;
    let t7696 = t1986 * t7695;
    let t7697 = t1994 * t7696;
    let t7698 = 0.53205749866622299248e-5 * t7697;
    let t7699 = t1249 * t687;
    (t7684, t7686, t7687, t7689, t7690, t7691, t7693, t7696, t7698, t7699)
}
