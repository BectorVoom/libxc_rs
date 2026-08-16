//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 930/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk930(t27120: f64, t739: f64, t7577: f64, t2001: f64, t2281: f64, t305: f64, t321: f64, t7720: f64, t495: f64, t511: f64, t7230: f64, t7231: f64, t9104: f64) -> (f64, f64, f64) {
    let t40027 = t739 * t7577 * t27120;
    let t40031 = t2001 * t305 * t2281 * t321;
    let t40032 = t7720 * t40031;
    let t40037 = t7230 * t7231 * t511 * t9104 * t495;
    (t40027, t40032, t40037)
}
