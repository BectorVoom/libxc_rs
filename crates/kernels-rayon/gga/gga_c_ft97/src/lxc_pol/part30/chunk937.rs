//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 937/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk937(t33425: f64, t683: f64, t173: f64, t33403: f64, t27616: f64, t6037: f64, t1614: f64, t218: f64, t679: f64, t24286: f64, t7470: f64, t6815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t140885 = t33425 * t683;
    let t140892 = t33403 * t173;
    let t140894 = t27616 * t140892 * t6037;
    let t140919 = t1614 * t218;
    let t140920 = t140919 * t679;
    let t140927 = t7470 * t24286;
    let t140929 = 0.75685073759570552987e-4_f64 * t6815 * t140927;
    (t140885, t140892, t140894, t140919, t140920, t140927, t140929)
}
