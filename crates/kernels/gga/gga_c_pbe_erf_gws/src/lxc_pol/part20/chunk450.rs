//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 450/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk450<F: Float>(t1952: F, t551: F, t553: F, t536: F, t547: F, t331: F, t535: F, t1369: F, t163: F, t148: F, t1371: F, t550: F) -> (F, F, F, F, F, F, F) {
    let t1955 = F::cast_from(0.65846301096364936273e-2_f64) * t1952 * t551 * t553;
    let t1958 = t536 * t547;
    let t1960 = t331 * t535;
    let t1962 = t1960 * t551 * t553;
    let t1964 = t1369 * t163;
    let t1966 = F::cast_from(0.31505407223141117834e-1_f64) * t148 * t1964;
    let t1969 = F::cast_from(0.39507780657818961764e-2_f64) * t550 * t1371 * t553;
    (t1955, t1958, t1960, t1962, t1964, t1966, t1969)
}
