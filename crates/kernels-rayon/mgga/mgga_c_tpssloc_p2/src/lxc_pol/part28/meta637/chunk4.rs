//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2036/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2036(t92161: f64, t92210: f64, t93275: f64, t93930: f64, t93978: f64, t94022: f64, t94061: f64, t94103: f64, t1404: f64, t7945: f64, t2105: f64, t5363: f64) -> (f64, f64, f64) {
    let t94106 = t92161 + t92210 + t93275 + t93930 + t93978 + t94022 + t94061 + t94103;
    let t94113 = 2.0_f64 * t7945 * t1404;
    let t94118 = 2.0_f64 * t5363 * t2105;
    (t94106, t94113, t94118)
}
