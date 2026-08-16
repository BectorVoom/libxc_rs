//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2000/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2000(t1877: f64, t2057: f64, t23792: f64, t23807: f64, t24191: f64, t24339: f64, t2522: f64, t25892: f64, t25898: f64, t25928: f64, t25938: f64, t25945: f64, t26563: f64, t28: f64, t7110: f64, t7845: f64, t84797: f64, t89843: f64, t89881: f64, t89928: f64, t89972: f64, t89987: f64, t92271: f64, t92295: f64, t92299: f64, t92990: f64, t93000: f64) -> f64 {
    let t93144 = 3.0_f64 * t2522 * t7110 * t25938 - 6.0_f64 * t26563 * t89928 - t1877 * t24339 * t25945 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t89881 + 3.0_f64 * t26563 * t89843 + 6.0_f64 * t92295 * t25892 - 3.0_f64 * t24191 * t89987 - 3.0_f64 * t24191 * t89972 - 3.0_f64 * t84797 * t25898 + 3.0_f64 * t2522 * t7845 * t23792 - t92299 + t1877 * t92990 * t28 / 2.0_f64 + 2.0_f64 * t92271 * t25928 + t1877 * t93000 * t23807;
    t93144
}
