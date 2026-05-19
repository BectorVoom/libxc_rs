//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1031/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1031<F: Float>(t10083: F, t11404: F, t406: F, t3898: F, t3913: F, t2381: F, t10138: F, t10141: F, t10148: F, t10201: F, t11383: F, t11391: F, t11396: F, t11401: F, t2380: F, t3185: F, t3206: F, t388: F, t3900: F, t404: F, t8285: F, t8319: F, t8340: F) -> (F, F, F) {
    let t11405 = t10083 * t11404;
    let t11406 = t406 * t11405;
    let t11409 = t3913 * t3898;
    let t11410 = t2381 * t11409;
    let t11416 = t10138 / F::new(18.0) - F::new(77.0) / F::new(162.0) * t11383 * t388 - F::cast_from(0.42874018118069736972e-3_f64) * t10141 + F::cast_from(0.13719685797782315831e-1_f64) * t8319 * t3900 - F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t11391 + F::cast_from(0.38586616306262763275e-2_f64) * t2380 * t11396 + F::cast_from(0.7622047665434619906e-3_f64) * t8285 - F::cast_from(0.42874018118069736972e-3_f64) * t404 * t11401 + F::cast_from(0.12862205435420921092e-2_f64) * t3185 * t11406 + F::cast_from(0.12862205435420921092e-2_f64) * t3206 * t11410 + F::new(11.0) / F::new(108.0) * t10148 + F::cast_from(0.42874018118069736972e-3_f64) * t10201 + t8340 / F::new(144.0);
    (t11405, t11409, t11416)
}
