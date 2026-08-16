//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 813/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk813(t2060: f64, t40940: f64, t903: f64, t15244: f64, t2604: f64, t41122: f64, t13954: f64, t5055: f64, t13958: f64, t2868: f64, t15272: f64, t69054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74598 = 0.8980681276397856423e-1_f64 * t903 * t2060 * t40940;
    let t74600 = 0.5987120850931904282e-1_f64 * t2604 * t15244;
    let t74603 = 0.8980681276397856423e-1_f64 * t903 * t2060 * t41122;
    let t74605 = 0.8980681276397856423e-1_f64 * t5055 * t13954;
    let t74609 = 0.5987120850931904282e-1_f64 * t2868 * t13958;
    let t74610 = t69054 * t15272;
    (t74598, t74600, t74603, t74605, t74609, t74610)
}
