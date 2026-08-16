//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 883/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk883(t2371: f64, t4199: f64, t1409: f64, t2517: f64, t707: f64, t1484: f64, t212: f64, t9523: f64, t2586: f64, t1489: f64, t9541: f64, t4134: f64, t9546: f64) -> (f64, f64, f64, f64, f64) {
    let t12943 = t4199 * t2371;
    let t12945 = t2517 * t1409;
    let t12946 = t707 * t12945;
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    let t12986 = t2586 * t12985;
    let t13010 = t9541 * t1489;
    let t13022 = t9546 * t4134;
    (t12943, t12946, t12986, t13010, t13022)
}
