//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 811/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk811(t9207: f64, t1772: f64, t7591: f64, t7602: f64, t7581: f64, t4998: f64, t9217: f64, t2013: f64, t9168: f64, t10886: f64, t9172: f64, t2012: f64, t23768: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24879 = t9207 * sigma2;
    let t24880 = t24879 * t1772;
    let t24908 = t7591 * t7602;
    let t24910 = t7581 * t7602;
    let t24912 = t4998 * t9217;
    let t24913 = t2013 * t24912;
    let t24920 = t4998 * t9168;
    let t24921 = t2013 * t24920;
    let t24925 = t10886 * t9172;
    let t24926 = t2013 * t24925;
    let t24967 = t2012 * t23768;
    (t24880, t24908, t24910, t24913, t24921, t24926, t24967)
}
