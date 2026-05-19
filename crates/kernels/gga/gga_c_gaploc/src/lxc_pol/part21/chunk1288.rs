//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1288/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1288<F: Float>(t14571: F, t32616: F, t20019: F, t25198: F, t7069: F, t29074: F, t29078: F, t23104: F, t3005: F, t7396: F, t20671: F, t24505: F, t28069: F) -> (F, F, F, F, F, F) {
    let t33929 = F::cast_from(0.15889106645266856297e0_f64) * t14571 * t32616;
    let t33932 = F::cast_from(0.23833659967900284446e0_f64) * t25198 * t20019 * t7069;
    let t33933 = F::cast_from(0.31952438294933958064e-1_f64) * t29074;
    let t33934 = F::cast_from(0.31952438294933958064e-1_f64) * t29078;
    let t33936 = t23104 * t3005 * t7396;
    let t33937 = F::cast_from(0.38342925953920749676e0_f64) * t33936;
    let t33942 = t28069 * t20671 * t24505;
    (t33929, t33932, t33933, t33934, t33937, t33942)
}
