//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 977/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk977<F: Float>(t118: F, t1986: F, t495: F, t571: F, t7717: F, t2001: F, t498: F, t7720: F, t1618: F, t1600: F, t11905: F, t2028: F, t2604: F, t36402: F, t36416: F, t36418: F, t36448: F, t36453: F, t40679: F, t40681: F, t40683: F, t40685: F, t40688: F, t40690: F, t8994: F) -> F {
    let t40694 = t1986 * t118 * t571 * t495;
    let t40695 = t7717 * t40694;
    let t40699 = t2001 * t118 * t571 * t498;
    let t40700 = t7720 * t40699;
    let t40702 = t1986 * t1618;
    let t40703 = t7720 * t40702;
    let t40705 = t1986 * t1600;
    let t40706 = t7720 * t40705;
    let t40714 = F::cast_from(0.20001418546446583934e0_f64) * t36402 + F::cast_from(0.54549323308490683458e-1_f64) * t36416 - F::cast_from(0.72732431077987577944e-1_f64) * t36418 - F::cast_from(0.41382249896261788303e-4_f64) * t40679 - F::cast_from(0.33105799917009430643e-4_f64) * t40681 - F::cast_from(0.25538759935978703638e-4_f64) * t40683 - F::cast_from(0.5987120850931904282e-1_f64) * t40685 - F::cast_from(0.2993560425465952141e-1_f64) * t40688 + F::cast_from(0.2993560425465952141e-1_f64) * t40690 + F::cast_from(0.1064114997332445985e-4_f64) * t40695 + F::cast_from(0.85129199786595678796e-5_f64) * t40700 - F::cast_from(0.25538759935978703638e-4_f64) * t40703 - F::cast_from(0.25538759935978703638e-4_f64) * t40706 - F::cast_from(0.11974241701863808564e0_f64) * t11905 * t2028 - F::cast_from(0.59590439850616975158e-4_f64) * t36448 - F::cast_from(0.19863479950205658386e-4_f64) * t36453 - F::cast_from(0.11974241701863808564e0_f64) * t2604 * t8994;
    t40714
}
