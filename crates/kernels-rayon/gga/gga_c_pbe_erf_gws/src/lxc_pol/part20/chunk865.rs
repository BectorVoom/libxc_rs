//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 865/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk865(t3074: f64, t8880: f64, t1133: f64, t2157: f64, t1105: f64, t874: f64, t1134: f64, t810: f64, t858: f64, t2407: f64, t2142: f64, t3120: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8881 = t3074 * t8880;
    let t8884 = t1133 * t2157;
    let t8890 = t1105 * t874;
    let t8895 = t1134 * t810;
    let t8896 = t858 * t8895;
    let t8897 = t2407 * t8896;
    let t8901 = 7.0_f64 / 144.0_f64 * t3120 * t2142;
    (t8881, t8884, t8890, t8895, t8897, t8901)
}
