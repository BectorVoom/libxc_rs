//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 844/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk844(t75123: f64, t7720: f64, t14020: f64, t14117: f64, t14123: f64, t21052: f64, t73712: f64, t68357: f64, t73717: f64, t15394: f64, t70548: f64, t2060: f64, t8794: f64) -> (f64, f64, f64, f64, f64) {
    let t75124 = t7720 * t75123;
    let t75134 = t21052 * t14020 * t14123 * t14117 * t73712;
    let t75137 = t68357 * t14117 * t73717;
    let t75139 = t70548 * t15394;
    let t75141 = t2060 * t8794;
    (t75124, t75134, t75137, t75139, t75141)
}
