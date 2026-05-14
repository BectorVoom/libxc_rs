//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 732/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk732<F: Float>(t311: F, t8950: F, t10: F, t3145: F, t1506: F, t8446: F, t8487: F, t1516: F, t3138: F, t1508: F, t3137: F, t1121: F, t1499: F, t3079: F, t1502: F, t530: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12042 = t311 * t8950;
    let t12068 = t10 * t3145;
    let t12075 = t8446 * t1506;
    let t12079 = t8487 * t1506;
    let t12098 = t1516 * t3138;
    let t12105 = t3137 * t1508;
    let t12106 = t1121 * t12105;
    let t12119 = t1499 * t3079;
    let t12121 = t530 * t1502;
    (t12042, t12068, t12075, t12079, t12098, t12105, t12106, t12119, t12121)
}
