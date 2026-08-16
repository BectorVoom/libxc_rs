//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1337/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1337<F: Float>(t3492: F, t6024: F, t16239: F, t3477: F, t14571: F, t32616: F, t20019: F, t25198: F, t7069: F, t29074: F, t29078: F, t23104: F, t3005: F, t7396: F) -> (F, F, F, F, F, F, F) {
    let t33922 = F::cast_from(0.11502877786176224903e2_f64) * t6024 * t3492;
    let t33927 = F::cast_from(0.71500979903700853338e0_f64) * t16239 * t3477;
    let t33929 = F::cast_from(0.15889106645266856297e0_f64) * t14571 * t32616;
    let t33932 = F::cast_from(0.23833659967900284446e0_f64) * t25198 * t20019 * t7069;
    let t33933 = F::cast_from(0.31952438294933958064e-1_f64) * t29074;
    let t33934 = F::cast_from(0.31952438294933958064e-1_f64) * t29078;
    let t33936 = t23104 * t3005 * t7396;
    (t33922, t33927, t33929, t33932, t33933, t33934, t33936)
}
