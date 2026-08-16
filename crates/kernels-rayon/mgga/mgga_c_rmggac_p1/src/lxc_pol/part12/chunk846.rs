//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 846/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk846(t4928: f64, t665: f64, t2060: f64, t5249: f64, t739: f64, t270: f64, t574: f64, t290: f64, t2010: f64, t7755: f64, t1664: f64, t7556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38809 = t665 * t4928;
    let t38812 = t2060 * t5249;
    let t38813 = t739 * t38812;
    let t38815 = t574 * t270;
    let t38816 = t290 * t38815;
    let t38818 = t2010 * t7755 * t38816;
    let t38819 = 0.72042316457491791906e-3_f64 * t38818;
    let t38820 = t1664 * t7556;
    (t38809, t38812, t38813, t38815, t38819, t38820)
}
