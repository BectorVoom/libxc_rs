//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 825/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk825<F: Float>(t8706: F, t8710: F, t8712: F, t8714: F, t8716: F, t8718: F, t8722: F, t7429: F, t7435: F, t7442: F, t7449: F, t8171: F, t8704: F, t8708: F, t8720: F) -> F {
    let t9261 = F::new(0.17149607247227894789e-2) * t8706;
    let t9263 = F::new(0.34299214494455789578e-2) * t8710;
    let t9264 = F::new(0.80031500487063509015e-2) * t8712;
    let t9265 = F::new(0.80031500487063509015e-2) * t8714;
    let t9266 = F::new(0.16006300097412701803e-1) * t8716;
    let t9267 = F::new(0.34299214494455789578e-2) * t8718;
    let t9269 = F::new(0.12862205435420921092e-2) * t8722;
    let t9271 = F::new(0.68598428988911579156e-2) * t8704 - t9261 - F::new(0.68598428988911579156e-2) * t8708 + t9263 + t9264 - t9265 + t9266 - t9267 - F::new(0.34299214494455789578e-2) * t8720 - t9269 - F::new(0.94344276868812456207e-3) * t7429 - t7435 - t7442 - t7449 - t8171;
    t9271
}
