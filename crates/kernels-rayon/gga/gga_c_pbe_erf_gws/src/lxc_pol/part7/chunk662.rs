//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 662/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk662(t4952: f64, t5264: f64, t4976: f64, t606: f64, t4939: f64, t25: f64, t4960: f64, t4965: f64, t5248: f64, t5250: f64, t5253: f64, t5256: f64, t5258: f64, t5260: f64) -> (f64, f64, f64) {
    let t5265 = t5264 * t4952;
    let t5268 = t606 * t4976;
    let t5271 = 0.11197407407407407407e0_f64 * t4939;
    let t5272 = 0.14396666666666666667e0_f64 * t4960 - 0.71983333333333333335e-1_f64 * t4965 - 0.26666666666666666667e-1_f64 * t5248 + 0.13333333333333333333e-1_f64 * t25 * t5250 - 0.66666666666666666666e-2_f64 * t25 * t5253 - 0.22222222222222222222e-1_f64 * t5256 + 0.13333333333333333334e-1_f64 * t5258 + 0.44444444444444444445e-2_f64 * t5260 - 0.29629629629629629629e-2_f64 * t25 * t5265 - 0.66666666666666666667e-2_f64 * t25 * t5268 - t5271;
    (t5265, t5268, t5272)
}
