//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 949/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk949<F: Float>(t452: F, t6454: F, t986: F, t29796: F, t83: F, t1339: F, t1871: F, t4436: F, t23323: F, t4612: F, t4572: F, t5710: F, t29603: F, t29606: F, t4458: F, t447: F) -> (F, F, F, F, F, F, F, F) {
    let t29862 = t452 * t986 * t6454;
    let t29865 = t83 * t29796;
    let t29869 = t1871 * t1339 * t4436;
    let t29872 = t23323 * t4612;
    let t29876 = t452 * t5710 * t4572;
    let t29879 = t83 * t29603;
    let t29882 = t83 * t29606;
    let t29888 = t447 * t1339 * t4458;
    (t29862, t29865, t29869, t29872, t29876, t29879, t29882, t29888)
}
