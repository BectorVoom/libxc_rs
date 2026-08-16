//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1214/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1214(t22852: f64, t3792: f64, t80786: f64, t80798: f64, t80749: f64, t80751: f64, t80753: f64, t80755: f64, t80757: f64, t80759: f64, t80761: f64, t80763: f64, t80767: f64, t80769: f64, t80773: f64, t80776: f64, t80780: f64, t80784: f64, t80789: f64, t80792: f64, t80794: f64, t80796: f64) -> f64 {
    let t80801 = t22852 * t80798 * t80786 * t3792;
    let t80803 = t80749 / 256.0_f64 - t80751 / 64.0_f64 + t80753 / 128.0_f64 - t80755 / 512.0_f64 - 5.0_f64 / 128.0_f64 * t80757 + t80759 / 128.0_f64 + 7.0_f64 / 48.0_f64 * t80761 - t80763 / 48.0_f64 - 0.2034786907144675699e0_f64 * t80767 + 0.25434836339308446238e-1_f64 * t80769 - 0.12111826828242117256e-2_f64 * t80773 - 35.0_f64 / 72.0_f64 * t80776 - 0.94875976821229918508e-2_f64 * t80780 + 0.50465945117675488567e-4_f64 * t80784 + 0.10093189023535097714e-3_f64 * t80789 - 0.15812662803538319751e-2_f64 * t80792 + 119.0_f64 / 2304.0_f64 * t80794 - 7.0_f64 / 768.0_f64 * t80796 - 0.20186378047070195427e-3_f64 * t80801;
    t80803
}
