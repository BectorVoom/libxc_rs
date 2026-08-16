//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1219/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1219(t3324: f64, t944: f64, t1105: f64, t13751: f64, t13756: f64, t14149: f64, t14153: f64, t14390: f64, t2051: f64, t2074: f64, t30098: f64, t3189: f64, t3946: f64, t4062: f64, t50825: f64, t50837: f64, t50846: f64, t52801: f64, t52810: f64, t52812: f64, t52816: f64, t52821: f64, t52823: f64) -> f64 {
    let t52829 = t3324 * t944;
    let t52833 = 3.0_f64 * t1105 * t3946 * t50825 + 12.0_f64 * t13751 * t13756 * t3189 - 2.0_f64 * t14149 * t3324 * t4062 + 4.0_f64 * t14153 * t4062 * t52829 + 3.0_f64 * t14390 * t2074 * t3946 + 2.0_f64 * t2051 * t4062 * t52816 - 12.0_f64 * t30098 * t52823 + 2.0_f64 * t50837 - t50846 + t52801 - t52810 - t52812 - t52821;
    t52833
}
