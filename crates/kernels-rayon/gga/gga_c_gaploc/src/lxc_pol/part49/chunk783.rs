//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 783/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk783(t12404: f64, t12405: f64, t12783: f64, t12784: f64, t12787: f64, t12788: f64, t12789: f64, t12790: f64, t12791: f64) -> f64 {
    let t13749 = t12783 + t12784 / 2.0_f64 + t12404 - t12405 - t12787 - t12788 + t12789 + t12790 + t12791;
    t13749
}
