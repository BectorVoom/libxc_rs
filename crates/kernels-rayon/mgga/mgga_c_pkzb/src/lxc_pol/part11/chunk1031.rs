//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1031/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1031(t10083: f64, t11404: f64, t406: f64, t3898: f64, t3913: f64, t2381: f64, t10138: f64, t10141: f64, t10148: f64, t10201: f64, t11383: f64, t11391: f64, t11396: f64, t11401: f64, t2380: f64, t3185: f64, t3206: f64, t388: f64, t3900: f64, t404: f64, t8285: f64, t8319: f64, t8340: f64) -> (f64, f64, f64) {
    let t11405 = t10083 * t11404;
    let t11406 = t406 * t11405;
    let t11409 = t3913 * t3898;
    let t11410 = t2381 * t11409;
    let t11416 = t10138 / 18.0_f64 - 77.0_f64 / 162.0_f64 * t11383 * t388 - 0.42874018118069736972e-3_f64 * t10141 + 0.13719685797782315831e-1_f64 * t8319 * t3900 - 0.12862205435420921092e-2_f64 * t2380 * t11391 + 0.38586616306262763275e-2_f64 * t2380 * t11396 + 0.7622047665434619906e-3_f64 * t8285 - 0.42874018118069736972e-3_f64 * t404 * t11401 + 0.12862205435420921092e-2_f64 * t3185 * t11406 + 0.12862205435420921092e-2_f64 * t3206 * t11410 + 11.0_f64 / 108.0_f64 * t10148 + 0.42874018118069736972e-3_f64 * t10201 + t8340 / 144.0_f64;
    (t11405, t11409, t11416)
}
