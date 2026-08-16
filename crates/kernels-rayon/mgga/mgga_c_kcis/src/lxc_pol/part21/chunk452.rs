//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 452/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk452(t2865: f64, t69: f64, t984: f64, t987: f64, t983: f64, t990: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t2866 = sigma0 * t2865;
    let t2867 = t2866 * t69;
    let t2870 = t984 * t987;
    let t2872 = t983 * t990;
    (t2866, t2867, t2870, t2872)
}
