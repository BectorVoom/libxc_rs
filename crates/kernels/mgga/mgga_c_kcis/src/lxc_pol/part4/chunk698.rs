//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 698/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk698<F: Float>(t486: F, t3999: F, t494: F, t1380: F, t286: F, t3951: F, t1378: F, t1368: F, t3969: F, t3972: F, t3975: F, t3981: F, t3986: F, t3991: F, t3995: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t495 = F::cast_from(0.0_f64) < t486;
    let t4000 = t494 * t3999;
    let t4001 = t1380 * t1380;
    let t4002 = t4000 * t4001;
    let t4003 = t286 * t4002;
    let t4007 = piecewise3::<F>(t495, t3951, -t3951);
    let t4008 = t1378 * t4007;
    let t4009 = t286 * t4008;
    let t4012 = -t3969 + t3972 / F::cast_from(432.0_f64) - t3975 / F::cast_from(144.0_f64) + t1368 * t3981 / F::cast_from(216.0_f64) - t1368 * t3986 / F::cast_from(144.0_f64) - t1368 * t3991 / F::cast_from(144.0_f64) + t1368 * t3995 / F::cast_from(288.0_f64) + t493 * t4003 / F::cast_from(48.0_f64) - t493 * t4009 / F::cast_from(96.0_f64);
    (t4001, t4002, t4003, t4007, t4008, t4009, t4012)
}
