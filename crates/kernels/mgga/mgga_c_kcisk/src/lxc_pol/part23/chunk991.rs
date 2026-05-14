//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 991/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk991<F: Float>(t20052: F, t403: F, t19132: F, t1390: F, t2168: F, t3278: F, t3937: F, t2075: F, t3988: F, t13482: F, t13487: F, t13493: F, t20027: F, t20036: F, t20039: F, t20043: F, t20048: F, t3935: F, t6201: F) -> (F, F) {
    let t20053 = t20052 * t403;
    let t20054 = t20053 * t19132;
    let t20057 = t2168 * t1390;
    let t20058 = t20057 * t3278;
    let t20059 = t3937 * t20058;
    let t20062 = t2075 * t3988;
    let t20063 = t3937 * t20062;
    let t20066 = -0.47975436576472845902e-1 * t3935 * t20027 - 0.35981577432354634426e-1 * t13493 * t6201 + 0.95950873152945691804e-1 * t13482 * t6201 - t20036 - 0.35981577432354634426e-1 * t3935 * t20039 - 0.17990788716177317213e-1 * t3935 * t20043 - 0.23987718288236422951e-1 * t3935 * t20048 - 0.11993859144118211475e-1 * t13487 - 0.1439263097294185377e0 * t3935 * t20054 + 0.35981577432354634426e-1 * t3935 * t20059 - 0.17990788716177317213e-1 * t3935 * t20063;
    (t20053, t20066)
}
