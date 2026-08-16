//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 647/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk647(t241: f64, t258: f64, t28097: f64, t1175: f64, t2574: f64, t6079: f64, t3977: f64, t6088: f64, t729: f64, t6852: f64, t773: f64, t265: f64, t27836: f64) -> (f64, f64, f64, f64, f64) {
    let t28417 = t241 * t28097 * t258;
    let t28422 = t2574 * t1175 * t6079;
    let t28426 = t729 * t3977 * t6088;
    let t28430 = t2574 * t773 * t6852;
    let t28434 = t2574 * t265 * t27836;
    (t28417, t28422, t28426, t28430, t28434)
}
