//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 923/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk923<F: Float>(t1859: F, t1979: F, t1982: F, t201: F, t446: F, t10050: F, t35470: F, t34960: F, t39333: F, t39339: F, t39341: F, t39345: F, t39370: F, t45361: F, t45363: F, t45365: F, t45367: F, t45371: F, t45374: F, t45381: F, t45385: F, t45389: F) -> F {
    let t45394 = t446 * t1859 * t201 * t1979 * t1982;
    let t45396 = t35470 * t10050;
    let t45399 = F::cast_from(0.12769379967989351819e-4_f64) * t45361 + F::cast_from(0.51077519871957407276e-4_f64) * t45363 - F::cast_from(0.76616279807936110914e-4_f64) * t45365 - F::cast_from(0.25538759935978703638e-4_f64) * t45367 + F::cast_from(0.25538759935978703638e-4_f64) * t45371 + F::cast_from(0.16260079888840015101e-2_f64) * t39333 - t39339 - F::cast_from(0.20455996240684006296e-1_f64) * t45374 + F::cast_from(0.68400385060046895006e-6_f64) * t39341 + F::cast_from(0.68400385060046895006e-6_f64) * t39345 - F::cast_from(0.14635184302277988245e0_f64) * t34960 + F::cast_from(0.1064114997332445985e-4_f64) * t45381 - F::cast_from(0.1064114997332445985e-4_f64) * t45385 - F::cast_from(0.17025839957319135759e-4_f64) * t45389 + F::cast_from(0.42564599893297839398e-5_f64) * t45394 + F::cast_from(0.11971293719990017331e-4_f64) * t45396 - F::cast_from(0.1616301098968908129e-5_f64) * t39370;
    t45399
}
