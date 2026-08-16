//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 587/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk587(t7678: f64, t1987: f64, t2191: f64, t1268: f64, t1986: f64, t675: f64, t1990: f64, t1274: f64, t1173: f64, t2189: f64, t674: f64, t1997: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7679 = 0.42564599893297839398e-5_f64 * t7678;
    let t7680 = t2191 * t1987;
    let t7681 = 0.25538759935978703638e-4_f64 * t7680;
    let t7682 = t1986 * t1268;
    let t7683 = t675 * t7682;
    let t7684 = 0.12769379967989351819e-4_f64 * t7683;
    let t7685 = t2191 * t1990;
    let t7686 = 0.85129199786595678796e-5_f64 * t7685;
    let t7687 = t1986 * t1274;
    let t7688 = t675 * t7687;
    let t7689 = 0.42564599893297839398e-5_f64 * t7688;
    let t7690 = t2189 * t1173;
    let t7691 = t7690 * t674;
    let t7692 = t7691 * t1997;
    (t7679, t7681, t7682, t7684, t7686, t7687, t7689, t7690, t7691, t7692)
}
