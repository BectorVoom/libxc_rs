//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1398/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1398(t12078: f64, t1397: f64, t1424: f64, t31068: f64, t34879: f64, t34881: f64, t34889: f64, t34893: f64, t34894: f64, t34897: f64, t34900: f64, t34903: f64, t34905: f64, t34910: f64, t34912: f64, t34914: f64, t34917: f64, t34919: f64) -> f64 {
    let t38770 = t1397 * t12078;
    let t38773 = t34879 + t34881 + t34889 - t34893 + t34894 - t31068 + t34897 + t34900 + t34903 - t34905 - 0.79445533226334281486e-1_f64 * t38770 * t1424 - t34910 + t34912 + t34914 + t34917 + t34919;
    t38773
}
