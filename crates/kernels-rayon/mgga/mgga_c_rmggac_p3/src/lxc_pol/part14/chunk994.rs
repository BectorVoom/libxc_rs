//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 994/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk994(t4895: f64, t665: f64, t39879: f64, t5271: f64, t262: f64, t40802: f64, t7835: f64, t35815: f64, t39662: f64, t39666: f64, t7788: f64, t40833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40960 = t665 * t4895;
    let t40963 = t5271 * t39879;
    let t40965 = t262 * t40802;
    let t40966 = t7835 * t40965;
    let t40967 = 0.36366215538993788972e-1_f64 * t40966;
    let t40968 = t35815 * t39662;
    let t40970 = t7788 * t39666;
    let t40972 = t262 * t40833;
    (t40960, t40963, t40965, t40967, t40968, t40970, t40972)
}
