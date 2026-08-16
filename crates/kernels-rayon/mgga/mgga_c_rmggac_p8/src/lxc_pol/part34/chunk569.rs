//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 569/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk569(t14585: f64, t2141: f64, t698: f64, t7262: f64, t235: f64) -> (f64, f64, f64) {
    let t14586 = t14585 * t2141;
    let t14587 = 0.13637330827122670864e-1_f64 * t14586;
    let t14588 = t7262 * t698;
    let t14589 = t235 * t14588;
    (t14587, t14588, t14589)
}
