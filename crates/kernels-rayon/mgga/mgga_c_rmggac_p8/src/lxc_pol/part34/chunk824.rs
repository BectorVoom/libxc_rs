//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 824/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk824(t14371: f64, t15214: f64, t11905: f64, t3072: f64, t14314: f64, t558: f64, t262: f64, t7192: f64, t1614: f64, t3080: f64, t15169: f64, t41886: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74797 = t14371 * t15214;
    let t74800 = 0.2993560425465952141e-1_f64 * t11905 * t3072;
    let t74801 = t14314 * t558;
    let t74802 = t262 * t74801;
    let t74803 = t7192 * t74802;
    let t74805 = t3080 * t1614;
    let t74806 = t262 * t74805;
    let t74807 = t7192 * t74806;
    let t74809 = t41886 * t15169;
    (t74797, t74800, t74801, t74802, t74803, t74805, t74806, t74807, t74809)
}
