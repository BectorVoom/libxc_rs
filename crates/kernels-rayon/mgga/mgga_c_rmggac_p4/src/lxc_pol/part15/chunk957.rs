//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 957/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk957(t2001: f64, t2281: f64, t305: f64, t551: f64, t7720: f64, t1364: f64, t35514: f64, t39899: f64, t39901: f64, t39923: f64, t39927: f64, t45864: f64, t45866: f64, t45869: f64, t45872: f64, t45874: f64, t45880: f64, t45884: f64, t5055: f64, t6421: f64, t6441: f64, t6473: f64, t665: f64, t8393: f64, t8396: f64, t903: f64) -> f64 {
    let t45889 = t2001 * t305 * t2281 * t551;
    let t45890 = t7720 * t45889;
    let t45892 = 0.17961362552795712846e0_f64 * t903 * t665 * t6441 - 0.23948483403727617128e0_f64 * t1364 * t665 * t6421 - 0.54549323308490683458e-1_f64 * t39899 + 0.51077519871957407276e-4_f64 * t45864 + 0.85129199786595678796e-5_f64 * t45866 + 0.19863479950205658386e-3_f64 * t39901 - 0.42564599893297839398e-5_f64 * t45869 + 0.33105799917009430643e-4_f64 * t35514 + 0.12769379967989351819e-4_f64 * t45872 - 0.12769379967989351819e-4_f64 * t45874 + 0.35922725105591425692e0_f64 * t5055 * t8393 - 0.47896966807455234256e0_f64 * t6473 * t8396 - 0.13637330827122670864e-1_f64 * t45880 - 0.13637330827122670864e-1_f64 * t45884 - 0.4726e1_f64 * t39923 - t39927 - 0.25538759935978703638e-4_f64 * t45890;
    t45892
}
