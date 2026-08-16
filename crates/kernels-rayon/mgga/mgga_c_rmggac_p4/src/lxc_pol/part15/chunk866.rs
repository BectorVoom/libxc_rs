//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 866/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk866(t10009: f64, t10012: f64, t10016: f64, t10020: f64, t38300: f64, t38304: f64, t38305: f64, t38306: f64, t38307: f64, t38308: f64, t7709: f64, t1970: f64, t1971: f64, t236: f64, t6149: f64) -> (f64, f64) {
    let t44568 = -t38300 + t10009 + t10012 + t7709 + t38304 + t10016 - t10020 + t38305 + t38306 + t38307 - t38308;
    let t44580 = t1970 * t1971 * t236 * t6149;
    (t44568, t44580)
}
