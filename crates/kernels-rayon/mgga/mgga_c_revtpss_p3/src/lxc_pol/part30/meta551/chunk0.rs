//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1990/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1990(t4423: f64, t836: f64, t231: f64, t50560: f64, t18632: f64, t2722: f64, t50474: f64, t14586: f64, t2645: f64, t50511: f64, t198: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51049 = t4423 * t836;
    let t51436 = t51049 * t231;
    let t51525 = t50560 * t231;
    let t51529 = t18632 * t836;
    let t51570 = t50474 * t2722;
    let t51574 = t14586 * t2722;
    let t51608 = t14586 * t2645;
    let t51698 = t50511 * t231;
    let t51780 = t198 * t2394;
    (t51436, t51525, t51529, t51570, t51574, t51608, t51698, t51780)
}
