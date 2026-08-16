//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 655/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk655(t288: f64, t49: f64, t108: f64, t4179: f64, t490: f64, t338: f64, t830: f64, t1330: f64, t28: f64, t7490: f64, t7552: f64, t1326: f64, t2016: f64, t7551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35253 = t49 * t288;
    let t35311 = t4179 * t108;
    let t35312 = t490 * t35311;
    let t35589 = t338 * t830;
    let t35613 = t28 * t1330;
    let t35620 = t7490 * t7552;
    let t35688 = t2016 * t7551 * t1326;
    (t35253, t35311, t35312, t35589, t35613, t35620, t35688)
}
