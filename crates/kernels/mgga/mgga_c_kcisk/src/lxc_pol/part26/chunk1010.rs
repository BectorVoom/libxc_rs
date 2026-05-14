//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1010/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1010<F: Float>(t25361: F, t26501: F, t26794: F, t26846: F, t26887: F, t26944: F, t27002: F, t27042: F, t504: F, t1458: F, t8185: F, t1520: F, t20919: F, t2282: F, t20922: F, t6244: F) -> (F, F, F, F, F, F) {
    let t27045 = t25361 + t26501 + t26794 + t26846 + t26887 + t26944 + t27002 + t27042;
    let t27046 = t27045 * t504;
    let t27047 = t8185 * t1458;
    let t27048 = t27047 * t1520;
    let t27050 = 2.0 * t20919 * t2282;
    let t27052 = 4.0 * t20922 * t6244;
    (t27045, t27046, t27047, t27048, t27050, t27052)
}
