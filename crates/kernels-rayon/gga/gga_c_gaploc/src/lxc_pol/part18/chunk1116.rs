//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1116/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1116(t2558: f64, t7589: f64, t943: f64, t2537: f64, t7064: f64, t7177: f64, t1842: f64, t21491: f64, t883: f64, t5538: f64, t7305: f64, t23296: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29233 = 0.64087718584518535698e-3_f64 * t943 * t7589 * t2558;
    let t29242 = 0.64087718584518535698e-3_f64 * t7064 * t2537 * t7177;
    let t29273 = 0.3845263115071112142e-2_f64 * t7064 * t1842 * t883 * t21491;
    let t29277 = t5538 * t883;
    let t29280 = 0.2563508743380741428e-2_f64 * t7064 * t29277 * t7305;
    let t29304 = 0.1281754371690370714e-2_f64 * t9647 * t23296 * t2558;
    (t29233, t29242, t29273, t29277, t29280, t29304)
}
