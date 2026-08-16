//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1192/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1192<F: Float>(t81072: F, t81074: F, t80749: F, t80751: F, t80753: F, t80755: F, t80757: F, t80759: F, t80761: F, t80763: F, t80767: F, t80769: F, t80773: F, t80776: F, t80780: F, t80784: F, t80789: F, t80792: F, t80794: F, t80796: F, t80801: F) -> (F, F, F) {
    let t84480 = F::cast_from(0.55440370401180965083e0_f64) * t81072;
    let t84481 = F::cast_from(0.3244175520728446583e0_f64) * t81074;
    let t84508 = t80749 / F::cast_from(128.0_f64) - t80751 / F::cast_from(32.0_f64) + t80753 / F::cast_from(64.0_f64) - t80755 / F::cast_from(256.0_f64) - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t80757 + t80759 / F::cast_from(64.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t80761 - t80763 / F::cast_from(24.0_f64) - F::cast_from(0.4069573814289351398e0_f64) * t80767 + F::cast_from(0.50869672678616892474e-1_f64) * t80769 - F::cast_from(0.24223653656484234512e-2_f64) * t80773 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t80776 - F::cast_from(0.18975195364245983701e-1_f64) * t80780 + F::cast_from(0.10093189023535097713e-3_f64) * t80784 + F::cast_from(0.20186378047070195427e-3_f64) * t80789 - F::cast_from(0.31625325607076639502e-2_f64) * t80792 + F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t80794 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t80796 - F::cast_from(0.40372756094140390854e-3_f64) * t80801;
    (t84480, t84481, t84508)
}
