//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1300/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1300(t22410: f64, t2409: f64, t3959: f64, t22192: f64, t3965: f64, t9220: f64, t26885: f64, t1146: f64, t13987: f64, t1178: f64, t371: f64, t3983: f64, t9258: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54624 = t3959 * t2409 * t22410;
    let t54627 = t3965 * t2409 * t22192;
    let t54629 = t3959 * t9220;
    let t54636 = t3965 * t2409 * t26885;
    let t54641 = t13987 * t1146;
    let t54649 = t3983 * t371 * t1178 * t9258;
    (t54624, t54627, t54629, t54636, t54641, t54649)
}
