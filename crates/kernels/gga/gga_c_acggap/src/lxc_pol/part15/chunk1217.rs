//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1217/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1217<F: Float>(t34293: F, t37013: F, t37014: F, t37016: F, t37017: F, t39141: F, t39143: F, t39145: F, t39147: F, t39151: F, t39155: F, t39160: F, t39162: F, t39167: F, t39169: F, t39171: F, t39173: F, t39176: F) -> F {
    let t41510 = -F::new(0.18868855373762491241e-2) * t39141 + F::new(0.13719685797782315831e-1) * t39143 + F::new(0.68598428988911579156e-2) * t39145 - F::new(0.34299214494455789578e-2) * t39147 - F::new(0.12579236915841660827e-2) * t39151 - F::new(0.15724046144802076034e-2) * t39155 + F::new(0.62896184579208304138e-3) * t39160 - F::new(0.12862205435420921092e-1) * t39162 - F::new(0.94344276868812456207e-3) * t39167 - t39169 / F::new(24.0) - t39171 / F::new(48.0) - F::new(0.80031500487063509015e-2) * t39173 + F::new(0.64025200389650807212e-1) * t34293 + t37013 - t37014 + t37016 + t37017 - F::new(0.21437009059034868486e-2) * t39176;
    t41510
}
