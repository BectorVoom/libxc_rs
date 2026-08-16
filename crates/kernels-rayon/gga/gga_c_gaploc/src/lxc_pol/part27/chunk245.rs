//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 245/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk245(t105: f64, t877: f64, t886: f64, t889: f64, t189: f64, t874: f64) -> (f64, f64) {
    let t892 = 0.28455006635676149599e-1_f64 * t105 * t877 + 0.11856252764865062333e-2_f64 * t886 - 0.28455006635676149599e-1_f64 * t105 * t889;
    let t894 = t189 * t874;
    (t892, t894)
}
