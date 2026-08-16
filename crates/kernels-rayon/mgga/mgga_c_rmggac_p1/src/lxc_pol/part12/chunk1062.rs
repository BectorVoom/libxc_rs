//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1062/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1062(t1979: f64, t1982: f64, t458: f64, t8607: f64, t1540: f64, t2150: f64, t36860: f64, t41954: f64, t41956: f64, t41958: f64, t41960: f64, t41962: f64, t41964: f64, t41969: f64, t41971: f64, t41973: f64, t41975: f64, t41978: f64, t41980: f64, t41983: f64, t41985: f64, t41989: f64) -> f64 {
    let t41993 = t8607 * t458 * t1979 * t1982;
    let t41995 = 0.25538759935978703638e-4_f64 * t41954 - 0.25538759935978703638e-4_f64 * t41956 + 0.1064114997332445985e-4_f64 * t41958 - 0.59590439850616975156e-4_f64 * t41960 + 0.85129199786595678796e-5_f64 * t41962 - 0.85129199786595678796e-5_f64 * t41964 - 0.39914139006212695214e-1_f64 * t1540 * t2150 + 0.99317399751028291929e-5_f64 * t36860 + 0.8980681276397856423e-1_f64 * t41969 - 0.85129199786595678796e-5_f64 * t41971 + 0.14967802127329760705e-1_f64 * t41973 + 0.8980681276397856423e-1_f64 * t41975 + t41978 - t41980 - 0.12769379967989351819e-4_f64 * t41983 + 0.12769379967989351819e-4_f64 * t41985 + 0.85129199786595678796e-5_f64 * t41989 + 0.85129199786595678796e-5_f64 * t41993;
    t41995
}
