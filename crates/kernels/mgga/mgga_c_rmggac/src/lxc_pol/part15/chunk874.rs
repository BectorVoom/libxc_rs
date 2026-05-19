//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 874/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk874<F: Float>(t39832: F, t8443: F, t41890: F, t39513: F, t8451: F, t44627: F, t44632: F, t44637: F, t44642: F, t44647: F, t44651: F, t44656: F, t44662: F, t44668: F, t44670: F, t44676: F, t44682: F, t44684: F, t44690: F) -> F {
    let t44692 = t39832 * t8443;
    let t44694 = t41890 * t8443;
    let t44696 = t8451 * t39513;
    let t44698 = F::cast_from(0.1064114997332445985e-4_f64) * t44627 + F::cast_from(0.42564599893297839398e-5_f64) * t44632 - F::cast_from(0.12769379967989351819e-4_f64) * t44637 + F::cast_from(0.12769379967989351819e-4_f64) * t44642 + F::cast_from(0.42564599893297839398e-5_f64) * t44647 - F::cast_from(0.85129199786595678796e-5_f64) * t44651 - F::cast_from(0.42564599893297839398e-5_f64) * t44656 - F::cast_from(0.25538759935978703638e-4_f64) * t44662 + F::cast_from(0.1064114997332445985e-4_f64) * t44668 + F::cast_from(0.85129199786595678796e-5_f64) * t44670 + F::cast_from(0.85129199786595678796e-5_f64) * t44676 + F::cast_from(0.85129199786595678796e-5_f64) * t44682 - F::cast_from(0.25538759935978703638e-4_f64) * t44684 - F::cast_from(0.25538759935978703638e-4_f64) * t44690 - F::cast_from(0.85129199786595678796e-5_f64) * t44692 - F::cast_from(0.85129199786595678796e-5_f64) * t44694 - F::cast_from(0.85129199786595678796e-5_f64) * t44696;
    t44698
}
