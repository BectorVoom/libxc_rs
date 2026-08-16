//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 634/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk634(t1923: f64, t1928: f64, t2730: f64, t2772: f64, t3517: f64, t3529: f64, t3533: f64, t3537: f64, t3539: f64, t3544: f64, t3548: f64) -> f64 {
    let t3577 = -0.17648625e1_f64 * t3529 + 0.3529725e1_f64 * t3533 + t1923 - 0.103295e1_f64 * t2730 + 0.1549425e1_f64 * t3517 + 0.31558125e0_f64 * t3537 + 0.6311625e0_f64 * t3539 + t1928 - 0.41678e0_f64 * t2772 + 0.312585e0_f64 * t3544 + 0.312585e0_f64 * t3548;
    t3577
}
