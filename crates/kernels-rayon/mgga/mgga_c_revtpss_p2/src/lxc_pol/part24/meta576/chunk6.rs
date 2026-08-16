//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1768/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1768(t24765: f64, t5192: f64, t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64) -> (f64, f64) {
    let t90602 = 0.4101607543286562663e4_f64 * t5192 * t24765;
    let t90614 = 0.11872222222222222222e0_f64 * t89824 - 0.42739999999999999999e0_f64 * t89828 - 0.52765432098765432099e-1_f64 * t89832 + 0.47488888888888888888e-1_f64 * t81156 - 0.14246666666666666667e0_f64 * t81158 + 0.47488888888888888888e-1_f64 * t68255 - 0.35616666666666666666e-1_f64 * t89839 - 0.47488888888888888888e-1_f64 * t89843 + 0.6411e0_f64 * t89847 + 0.10685e0_f64 * t89851 + 0.14246666666666666667e0_f64 * t89855;
    (t90602, t90614)
}
