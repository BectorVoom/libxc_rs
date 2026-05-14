//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 930/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk930<F: Float>(t1979: F, t1982: F, t458: F, t8601: F, t8607: F, t1540: F, t2150: F, t36860: F, t41954: F, t41956: F, t41958: F, t41960: F, t41962: F, t41964: F, t41969: F, t41971: F, t41973: F, t41975: F, t41978: F, t41980: F, t41983: F, t41985: F) -> (F,) {
    let t41989 = t8601 * t458 * t1979 * t1982;
    let t41993 = t8607 * t458 * t1979 * t1982;
    let t41995 = 0.25538759935978703638e-4 * t41954 - 0.25538759935978703638e-4 * t41956 + 0.1064114997332445985e-4 * t41958 - 0.59590439850616975156e-4 * t41960 + 0.85129199786595678796e-5 * t41962 - 0.85129199786595678796e-5 * t41964 - 0.39914139006212695214e-1 * t1540 * t2150 + 0.99317399751028291929e-5 * t36860 + 0.8980681276397856423e-1 * t41969 - 0.85129199786595678796e-5 * t41971 + 0.14967802127329760705e-1 * t41973 + 0.8980681276397856423e-1 * t41975 + t41978 - t41980 - 0.12769379967989351819e-4 * t41983 + 0.12769379967989351819e-4 * t41985 + 0.85129199786595678796e-5 * t41989 + 0.85129199786595678796e-5 * t41993;
    (t41995,)
}
