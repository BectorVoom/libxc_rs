//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 541/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk541(t41: f64, t4971: f64, t2009: f64, t2630: f64, t2629: f64, t1772: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t7568 = t41 * t4971;
    let t7578 = t2630 * t2009;
    let t7580 = t2629 * sigma2;
    let t7581 = t7580 * t1772;
    (t7568, t7578, t7580, t7581)
}
