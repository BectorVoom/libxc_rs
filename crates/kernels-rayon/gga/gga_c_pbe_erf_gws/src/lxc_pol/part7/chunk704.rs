//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 704/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk704(t1452: f64, t532: f64, t145: f64, t169: f64, t242: f64, t4867: f64, t5700: f64, t5703: f64, t5707: f64, t5710: f64, t5713: f64, t5717: f64, t5718: f64, t5723: f64, t5726: f64, t5730: f64, t5732: f64, t5733: f64) -> f64 {
    let t5735 = t532 * t1452;
    let t5739 = t5700 - 0.42447554366239164361e0_f64 * t5703 - t5707 + 0.15917832887339686635e0_f64 * t5710 + 0.3183566577467937327e0_f64 * t5713 + t5717 - 0.31835665774679373271e-1_f64 * t169 * t5718 * t242 - 0.95506997324038119813e-1_f64 * t5723 - 0.95506997324038119813e-1_f64 * t5726 - t5730 - t5732 + 0.9598512193592288454e0_f64 * t5733 - 0.3199504064530762818e0_f64 * t5735 + 0.533250677421793803e-1_f64 * t145 * t4867;
    t5739
}
