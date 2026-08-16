//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 999/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk999(t1286: f64, t34619: f64, t376: f64, t100089: f64, t101983: f64, t1308: f64, t136016: f64, t136018: f64, t136037: f64, t136041: f64, t136077: f64, t1564: f64, t22873: f64, t22935: f64, t25861: f64, t26493: f64, t28: f64, t32406: f64, t34787: f64, t5501: f64, t5507: f64, t6414: f64, t925: f64) -> f64 {
    let t144350 = t1286 * t376 * t34619;
    let t144372 = t136016 / 54.0_f64 - 2.0_f64 / 3.0_f64 * t6414 * t32406 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t5507 * t101983 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t22873 * t25861 + t144350 / 9.0_f64 + t1286 * t28 * t1308 * t26493 / 3.0_f64 - t5501 * t1564 * t136077 * t925 / 9.0_f64 - t22935 * t34787 / 9.0_f64 - t5501 * t1564 * t136018 * t925 / 9.0_f64 - t136037 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t136041 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t5507 * t100089;
    t144372
}
