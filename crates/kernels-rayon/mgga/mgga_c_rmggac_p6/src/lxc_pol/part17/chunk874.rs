//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 874/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk874(t39832: f64, t8443: f64, t41890: f64, t39513: f64, t8451: f64, t44627: f64, t44632: f64, t44637: f64, t44642: f64, t44647: f64, t44651: f64, t44656: f64, t44662: f64, t44668: f64, t44670: f64, t44676: f64, t44682: f64, t44684: f64, t44690: f64) -> f64 {
    let t44692 = t39832 * t8443;
    let t44694 = t41890 * t8443;
    let t44696 = t8451 * t39513;
    let t44698 = 0.1064114997332445985e-4_f64 * t44627 + 0.42564599893297839398e-5_f64 * t44632 - 0.12769379967989351819e-4_f64 * t44637 + 0.12769379967989351819e-4_f64 * t44642 + 0.42564599893297839398e-5_f64 * t44647 - 0.85129199786595678796e-5_f64 * t44651 - 0.42564599893297839398e-5_f64 * t44656 - 0.25538759935978703638e-4_f64 * t44662 + 0.1064114997332445985e-4_f64 * t44668 + 0.85129199786595678796e-5_f64 * t44670 + 0.85129199786595678796e-5_f64 * t44676 + 0.85129199786595678796e-5_f64 * t44682 - 0.25538759935978703638e-4_f64 * t44684 - 0.25538759935978703638e-4_f64 * t44690 - 0.85129199786595678796e-5_f64 * t44692 - 0.85129199786595678796e-5_f64 * t44694 - 0.85129199786595678796e-5_f64 * t44696;
    t44698
}
