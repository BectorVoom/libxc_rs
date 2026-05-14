//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 818/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk818<F: Float>(t1639: F, t649: F, t1642: F, t7506: F, t7115: F, t4908: F, t616: F, t5417: F, t5418: F, t5423: F, t5429: F, t5430: F, t5433: F, t5436: F, t7740: F, t7742: F, t7744: F, t7749: F, t7750: F, t7753: F, t7755: F, t7757: F) -> (F, F, F) {
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7760 = t7759 * t7506;
    let t7762 = 8.0 / 27.0 * t7115 * t7760;
    let t7764 = 4.0 / 15.0 * t616 * t4908;
    let t7768 = -t7740 + t7742 - t7744 + t7749 + t7750 - t7753 + t7755 + t7757 - t7762 - t7764 + 2.0 / 3.0 * t5417 + 0.2431111111111111111e0 * t5418 + t5423 + t5429 + 8.0 / 9.0 * t5430 + t5433 + t5436;
    (t7762, t7764, t7768)
}
