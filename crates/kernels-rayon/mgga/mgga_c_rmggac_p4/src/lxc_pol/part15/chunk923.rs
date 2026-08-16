//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 923/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk923(t1859: f64, t1979: f64, t1982: f64, t201: f64, t446: f64, t10050: f64, t35470: f64, t34960: f64, t39333: f64, t39339: f64, t39341: f64, t39345: f64, t39370: f64, t45361: f64, t45363: f64, t45365: f64, t45367: f64, t45371: f64, t45374: f64, t45381: f64, t45385: f64, t45389: f64) -> f64 {
    let t45394 = t446 * t1859 * t201 * t1979 * t1982;
    let t45396 = t35470 * t10050;
    let t45399 = 0.12769379967989351819e-4_f64 * t45361 + 0.51077519871957407276e-4_f64 * t45363 - 0.76616279807936110914e-4_f64 * t45365 - 0.25538759935978703638e-4_f64 * t45367 + 0.25538759935978703638e-4_f64 * t45371 + 0.16260079888840015101e-2_f64 * t39333 - t39339 - 0.20455996240684006296e-1_f64 * t45374 + 0.68400385060046895006e-6_f64 * t39341 + 0.68400385060046895006e-6_f64 * t39345 - 0.14635184302277988245e0_f64 * t34960 + 0.1064114997332445985e-4_f64 * t45381 - 0.1064114997332445985e-4_f64 * t45385 - 0.17025839957319135759e-4_f64 * t45389 + 0.42564599893297839398e-5_f64 * t45394 + 0.11971293719990017331e-4_f64 * t45396 - 0.1616301098968908129e-5_f64 * t39370;
    t45399
}
