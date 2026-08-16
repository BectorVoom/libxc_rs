//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2272/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2272(t2113: f64, t2363: f64, t12557: f64, t1459: f64, t1774: f64, t24543: f64, t24545: f64, t24932: f64, t27888: f64, t4028: f64, t4037: f64, t4073: f64, t652: f64, t7266: f64, t8103: f64, t85428: f64, t90421: f64, t90428: f64, t90434: f64, t90436: f64, t90440: f64, t90444: f64, t90447: f64, t90450: f64, t90454: f64, t90456: f64) -> (f64, f64) {
    let t94248 = t2113 * t2363;
    let t94257 = -2.0_f64 * t2363 * t652 * t8103 - 2.0_f64 * t12557 * t7266 - 2.0_f64 * t1459 * t85428 - 2.0_f64 * t1459 * t94248 - t1774 * t24543 - 4.0_f64 * t24545 * t4028 - 4.0_f64 * t24932 * t4073 - 4.0_f64 * t27888 * t4037 - 4.0_f64 * t27888 * t4073 + t90421 - t90428 + t90434 - t90436 + t90440 + t90444 + t90447 - t90450 - t90454 - t90456;
    (t94248, t94257)
}
