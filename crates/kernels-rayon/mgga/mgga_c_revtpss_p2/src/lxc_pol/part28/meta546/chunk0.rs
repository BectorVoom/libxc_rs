//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1995/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1995(t14749: f64, t221: f64, t14767: f64, t4423: f64, t836: f64, t231: f64, t50560: f64, t18632: f64, t2722: f64, t50474: f64, t14586: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50789 = t221 * t14749;
    let t50931 = t221 * t14767;
    let t51049 = t4423 * t836;
    let t51436 = t51049 * t231;
    let t51525 = t50560 * t231;
    let t51529 = t18632 * t836;
    let t51570 = t50474 * t2722;
    let t51574 = t14586 * t2722;
    let t51608 = t14586 * t2645;
    (t50789, t50931, t51436, t51525, t51529, t51570, t51574, t51608)
}
