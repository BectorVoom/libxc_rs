//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1445/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1445(t28987: f64, t28990: f64, t33853: f64, t33857: f64, t33859: f64, t33861: f64, t33863: f64, t33865: f64, t33867: f64, t33869: f64, t33872: f64, t33878: f64, t33881: f64, t33883: f64, t33891: f64, t33892: f64) -> f64 {
    let t39302 = -t33853 - t33857 - t33859 - t33861 + t33863 + t33865 + t33867 + t33869 - t33872 + t33878 + t33881 + t33883 - t33891 - 0.10224780254378866581e1_f64 * t28987 + 0.53964118009221795842e0_f64 * t28990 - t33892;
    t39302
}
