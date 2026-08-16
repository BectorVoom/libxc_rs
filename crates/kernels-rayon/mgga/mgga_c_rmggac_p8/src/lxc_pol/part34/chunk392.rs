//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 392/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk392(t2118: f64, t7645: f64, t344: f64, t830: f64, t1173: f64, t2189: f64, t2064: f64, t321: f64, t201: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7656 = t2118 * t7645;
    let t7662 = t344 * t830;
    let t7663 = 0.64905642291407286545e-3_f64 * t7662;
    let t7690 = t2189 * t1173;
    let t7707 = t2064 * t321;
    let t7715 = t201 * t1173;
    (t7656, t7662, t7663, t7690, t7707, t7715)
}
