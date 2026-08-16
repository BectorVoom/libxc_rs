//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1334/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1334(t2737: f64, t9802: f64, t221: f64, t2485: f64, t2754: f64, t2484: f64, t2749: f64, t836: f64, t853: f64, t2662: f64, t2661: f64, t2646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10826 = 0.45738002528356795401e-4_f64 * t9802 * t2737;
    let t10832 = t2485 * t221 * t2754;
    let t10833 = t2484 * t10832;
    let t10836 = t853 * t836 * t2749;
    let t10837 = t2662 * t10836;
    let t10838 = t2661 * t10837;
    let t10841 = t2485 * t221 * t2646;
    (t10826, t10832, t10833, t10836, t10838, t10841)
}
