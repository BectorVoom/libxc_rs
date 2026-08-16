//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 770/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk770(t265: f64, t839: f64, t262: f64, t7829: f64, t794: f64, t7844: f64, t7667: f64, t874: f64, t321: f64, t7617: f64, t5271: f64, t2079: f64, t352: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35884 = t265 * t839;
    let t35885 = t262 * t35884;
    let t35886 = t7829 * t35885;
    let t35888 = t265 * t794;
    let t35889 = t262 * t35888;
    let t35890 = t7844 * t35889;
    let t35906 = t874 * t7667;
    let t35917 = t7617 * t321;
    let t35918 = t5271 * t35917;
    let t35922 = t2079 * t262 * t830 * t352;
    (t35884, t35885, t35886, t35888, t35889, t35890, t35906, t35917, t35918, t35922)
}
