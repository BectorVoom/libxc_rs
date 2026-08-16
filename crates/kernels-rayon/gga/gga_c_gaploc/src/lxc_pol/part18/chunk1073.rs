//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1073/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1073(t10012: f64, t8669: f64, t2975: f64, t6081: f64, t2925: f64, t723: f64, t1022: f64, t1880: f64, t2021: f64, t8752: f64, t2101: f64, t24350: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24549 = t10012 * t8669;
    let t24554 = t6081 * t2975;
    let t24586 = t2925 * t723;
    let t24644 = t1022 * t1880;
    let t24657 = t2021 * t8752;
    let t24660 = t2101 * t2925;
    let t24722 = t739 * t24350;
    (t24549, t24554, t24586, t24644, t24657, t24660, t24722)
}
