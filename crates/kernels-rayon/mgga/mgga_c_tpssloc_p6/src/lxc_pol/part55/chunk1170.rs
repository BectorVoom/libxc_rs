//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1170/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1170(t112: f64, t32629: f64, t111: f64, t8919: f64, t193: f64, t8421: f64, t25374: f64, t86716: f64, t200: f64, t8365: f64, t25: f64, t25353: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118354 = t32629 * t112;
    let t118365 = t8919 * t111;
    let t118376 = t193 * t8421;
    let t118377 = t86716 * t25374;
    let t118381 = t193 * t200 * t8365;
    let t118387 = t25 * t25353;
    (t118354, t118365, t118376, t118377, t118381, t118387)
}
