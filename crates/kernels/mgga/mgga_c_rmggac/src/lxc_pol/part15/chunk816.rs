//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 816/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk816<F: Float>(t2281: F, t34975: F, t35039: F, t9145: F, t16503: F, t38508: F, t8420: F, t1859: F, t1979: F, t1982: F, t201: F, t446: F, t10050: F, t35470: F, t34960: F, t39333: F, t39339: F, t39341: F, t39345: F, t39370: F, t45361: F, t45363: F, t45365: F, t45367: F, t45371: F, t45374: F, t45381: F) -> (F,) {
    let t45385 = t34975 * t35039 * t2281 * t9145;
    let t45389 = t16503 * t38508 * t2281 * t8420;
    let t45394 = t446 * t1859 * t201 * t1979 * t1982;
    let t45396 = t35470 * t10050;
    let t45399 = 0.12769379967989351819e-4 * t45361 + 0.51077519871957407276e-4 * t45363 - 0.76616279807936110914e-4 * t45365 - 0.25538759935978703638e-4 * t45367 + 0.25538759935978703638e-4 * t45371 + 0.16260079888840015101e-2 * t39333 - t39339 - 0.20455996240684006296e-1 * t45374 + 0.68400385060046895006e-6 * t39341 + 0.68400385060046895006e-6 * t39345 - 0.14635184302277988245e0 * t34960 + 0.1064114997332445985e-4 * t45381 - 0.1064114997332445985e-4 * t45385 - 0.17025839957319135759e-4 * t45389 + 0.42564599893297839398e-5 * t45394 + 0.11971293719990017331e-4 * t45396 - 0.1616301098968908129e-5 * t39370;
    (t45399,)
}
