//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1479/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1479(t117687: f64, t120807: f64, t120809: f64, t120818: f64, t122804: f64, t122806: f64, t122808: f64, t122817: f64, t2039: f64, t24972: f64, t27170: f64, t27281: f64, t32406: f64, t4072: f64, t5376: f64, t7235: f64, t7423: f64, t96311: f64, t96334: f64) -> f64 {
    let t125043 = 27.0_f64 * t117687 * t5376 + t120807 + t122804 + t122806 + t122808 + t120809 + 0.135e2_f64 * t96311 * t2039 + t122817 + 27.0_f64 * t96334 * t7235 + 0.135e2_f64 * t7423 * t27170 + t120818 + 27.0_f64 * t24972 * t27281 + 0.135e2_f64 * t32406 * t4072;
    t125043
}
