//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 334/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk334(t2009: f64, t782: f64, t780: f64, t1772: f64, sigma2: f64) -> (f64, f64, f64) {
    let t2011 = 0.89953943580886586067e-2_f64 * t782 * t2009;
    let t2012 = t780 * sigma2;
    let t2013 = t2012 * t1772;
    (t2011, t2012, t2013)
}
