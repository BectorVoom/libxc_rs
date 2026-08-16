//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 964/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk964(t10874: f64, t2741: f64, t2753: f64, t3563: f64, t582: f64, t616: f64, t10863: f64, t10866: f64, t10870: f64, t10873: f64, t5384: f64, t5387: f64, t5417: f64, t7715: f64, t7728: f64, t7732: f64, t7734: f64, t7736: f64, t7753: f64, t7757: f64) -> (f64, f64, f64, f64) {
    let t10875 = 8.0_f64 / 45.0_f64 * t10874;
    let t10876 = t2741 * t2753;
    let t10877 = 16.0_f64 / 45.0_f64 * t10876;
    let t10878 = t582 * t3563;
    let t10879 = t616 * t10878;
    let t10880 = 8.0_f64 / 45.0_f64 * t10879;
    let t10882 = t10863 + t10866 - t7715 - t5384 + t5387 + t7728 + t7732 + 2.0_f64 / 3.0_f64 * t7734 + 0.2431111111111111111e0_f64 * t7736 - t10870 - t10873 - t10875 - t7753 + t7757 + t10877 + t10880 + t5417 / 3.0_f64;
    (t10875, t10877, t10880, t10882)
}
