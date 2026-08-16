//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1148/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1148(t14797: f64, t3068: f64, t3990: f64, t3989: f64, t3070: f64, t3965: f64, t3062: f64, t3959: f64, t1167: f64, t810: f64, t944: f64, t1105: f64, t14161: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14799 = t3990 * t14797 * t3068;
    let t14800 = t3989 * t14799;
    let t14806 = t3965 * t3070;
    let t14812 = t3959 * t3062;
    let t14825 = t1167 * t810;
    let t14831 = t1167 * t944;
    let t14843 = t14161 * t1105;
    (t14799, t14800, t14806, t14812, t14825, t14831, t14843)
}
