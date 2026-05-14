//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 779/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk779<F: Float>(t44627: F, t44632: F, t44637: F, t44642: F, t44647: F, t44651: F, t44656: F, t44662: F, t44668: F, t44670: F, t44676: F, t44682: F, t44684: F, t44690: F, t44692: F, t44694: F, t44696: F) -> (F,) {
    let t44698 = 0.1064114997332445985e-4 * t44627 + 0.42564599893297839398e-5 * t44632 - 0.12769379967989351819e-4 * t44637 + 0.12769379967989351819e-4 * t44642 + 0.42564599893297839398e-5 * t44647 - 0.85129199786595678796e-5 * t44651 - 0.42564599893297839398e-5 * t44656 - 0.25538759935978703638e-4 * t44662 + 0.1064114997332445985e-4 * t44668 + 0.85129199786595678796e-5 * t44670 + 0.85129199786595678796e-5 * t44676 + 0.85129199786595678796e-5 * t44682 - 0.25538759935978703638e-4 * t44684 - 0.25538759935978703638e-4 * t44690 - 0.85129199786595678796e-5 * t44692 - 0.85129199786595678796e-5 * t44694 - 0.85129199786595678796e-5 * t44696;
    (t44698,)
}
