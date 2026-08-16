//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 448/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk448<F: Float>(t164: F, t762: F, t1597: F, t528: F, t547: F, t147: F, t837: F, t551: F, t553: F, t536: F, t331: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t1947 = F::cast_from(0.63010814446282235668e-1_f64) * t762 * t164;
    let t1948 = t1597 * t164;
    let t1951 = F::cast_from(0.63010814446282235668e-1_f64) * t528 * t547;
    let t1952 = t837 * t147;
    let t1955 = F::cast_from(0.65846301096364936273e-2_f64) * t1952 * t551 * t553;
    let t1958 = t536 * t547;
    let t1960 = t331 * t535;
    (t1947, t1948, t1951, t1952, t1955, t1958, t1960)
}
