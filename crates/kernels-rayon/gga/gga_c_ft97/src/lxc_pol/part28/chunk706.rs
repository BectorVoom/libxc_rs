//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 706/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk706(t144: f64, t26599: f64, t1882: f64, t6645: f64, t6653: f64, t23548: f64, t3424: f64, t9144: f64, t3429: f64, t13220: f64, t1384: f64, t1570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27199 = t144 * t26599;
    let t27203 = t1882 * t6645;
    let t27205 = t1882 * t6653;
    let t27207 = t23548 * t3424;
    let t27208 = t9144 * t27207;
    let t27211 = t23548 * t3429;
    let t27212 = t13220 * t27211;
    let t27215 = t1384 * t1570;
    (t27199, t27203, t27205, t27207, t27208, t27211, t27212, t27215)
}
