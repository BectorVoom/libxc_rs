//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 954/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk954<F: Float>(t11409: F, t2381: F, t10138: F, t10141: F, t10148: F, t10201: F, t11383: F, t11391: F, t11396: F, t11401: F, t11406: F, t2380: F, t3185: F, t3206: F, t388: F, t3900: F, t404: F, t8285: F, t8319: F, t8340: F) -> (F,) {
    let t11410 = t2381 * t11409;
    let t11416 = t10138 / 18.0 - 77.0 / 162.0 * t11383 * t388 - 0.42874018118069736972e-3 * t10141 + 0.13719685797782315831e-1 * t8319 * t3900 - 0.12862205435420921092e-2 * t2380 * t11391 + 0.38586616306262763275e-2 * t2380 * t11396 + 0.7622047665434619906e-3 * t8285 - 0.42874018118069736972e-3 * t404 * t11401 + 0.12862205435420921092e-2 * t3185 * t11406 + 0.12862205435420921092e-2 * t3206 * t11410 + 11.0 / 108.0 * t10148 + 0.42874018118069736972e-3 * t10201 + t8340 / 144.0;
    (t11416,)
}
