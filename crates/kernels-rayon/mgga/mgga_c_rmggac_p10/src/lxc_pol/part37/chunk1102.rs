//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1102/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1102(t305: f64, t326: f64, t77875: f64, t77878: f64, t77881: f64, t77883: f64, t77884: f64, t77887: f64, t77888: f64, t77889: f64, t77898: f64, t77899: f64, t77900: f64, t77904: f64, t80280: f64, t80341: f64) -> f64 {
    let t80426 = 0.59871208509319042821e-1_f64 * t305 * t80341 - 0.59871208509319042821e-1_f64 * t326 * t80280 - t77875 - t77878 - t77881 - t77883 - t77884 + t77887 + t77888 + t77889 - t77898 + t77899 + t77900 - t77904;
    t80426
}
