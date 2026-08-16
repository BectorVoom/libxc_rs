//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1192/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1192(t81072: f64, t81074: f64, t80749: f64, t80751: f64, t80753: f64, t80755: f64, t80757: f64, t80759: f64, t80761: f64, t80763: f64, t80767: f64, t80769: f64, t80773: f64, t80776: f64, t80780: f64, t80784: f64, t80789: f64, t80792: f64, t80794: f64, t80796: f64, t80801: f64) -> (f64, f64, f64) {
    let t84480 = 0.55440370401180965083e0_f64 * t81072;
    let t84481 = 0.3244175520728446583e0_f64 * t81074;
    let t84508 = t80749 / 128.0_f64 - t80751 / 32.0_f64 + t80753 / 64.0_f64 - t80755 / 256.0_f64 - 5.0_f64 / 64.0_f64 * t80757 + t80759 / 64.0_f64 + 7.0_f64 / 24.0_f64 * t80761 - t80763 / 24.0_f64 - 0.4069573814289351398e0_f64 * t80767 + 0.50869672678616892474e-1_f64 * t80769 - 0.24223653656484234512e-2_f64 * t80773 - 35.0_f64 / 36.0_f64 * t80776 - 0.18975195364245983701e-1_f64 * t80780 + 0.10093189023535097713e-3_f64 * t80784 + 0.20186378047070195427e-3_f64 * t80789 - 0.31625325607076639502e-2_f64 * t80792 + 119.0_f64 / 1152.0_f64 * t80794 - 7.0_f64 / 384.0_f64 * t80796 - 0.40372756094140390854e-3_f64 * t80801;
    (t84480, t84481, t84508)
}
