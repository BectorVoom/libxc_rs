//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 825/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk825<F: Float>(t7346: F, t8952: F, t1165: F, t604: F, t8901: F, t7337: F, t7771: F, t7773: F, t7776: F, t7782: F, t7788: F, t7789: F, t7797: F, t7801: F, t7803: F, t7806: F, t8943: F, t8945: F, t8949: F) -> (F, F) {
    let t8953 = t7346 * t8952;
    let t8956 = t1165 * t604 * t8901;
    let t8957 = t7337 * t8956;
    let t8959 = -F::new(0.3572834843172478081e-3) * t7771 - F::new(0.64311027177104605458e-3) * t7773 - t7776 + t7782 - t7788 + F::new(0.10718504529517434243e-3) * t7789 + F::new(0.7145669686344956162e-4) * t7797 + t7801 - t7803 - t7806 + t8943 / F::new(96.0) - F::new(7.0) / F::new(288.0) * t8945 + F::new(0.21437009059034868486e-3) * t8949 - F::new(0.15724046144802076034e-3) * t8953 - F::new(0.7862023072401038017e-3) * t8957;
    (t8956, t8959)
}
