//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 356/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk356(t2574: f64, t265: f64, t6079: f64, t1424: f64, t729: f64, t773: f64, t766: f64, t762: f64, t6061: f64, t1445: f64, t681: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6081 = t2574 * t265 * t6079;
    let t6085 = t729 * t773 * t1424;
    let t6088 = t1424 * t766;
    let t6090 = t729 * t762 * t6088;
    let t6094 = t729 * t265 * t6061;
    let t6099 = t89 * t681 * t1445 / 9.0_f64;
    (t6081, t6085, t6088, t6090, t6094, t6099)
}
