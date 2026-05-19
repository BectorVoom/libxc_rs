//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1234/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1234<F: Float>(t32621: F, t32622: F, t32627: F, t32628: F, t35034: F, t35043: F, t37361: F, t37362: F, t37365: F, t39653: F, t39658: F, t39661: F, t39665: F, t39669: F, t39673: F, t39675: F, t39679: F, t39683: F) -> F {
    let t41748 = F::cast_from(0.13719685797782315831e-1_f64) * t39653 - t32621 - t32622 + t35034 - t32627 - t32628 + F::new(0.1528125e-1) * t39658 + t39661 / F::new(12.0) + t37361 + t37362 - F::cast_from(0.64311027177104605458e-2_f64) * t39665 + F::cast_from(0.10718504529517434243e-2_f64) * t39669 - F::new(35.0) / F::new(54.0) * t35043 - F::cast_from(0.14291339372689912324e-3_f64) * t39673 - t37365 + F::cast_from(0.31448092289604152069e-3_f64) * t39675 + F::cast_from(0.31448092289604152069e-3_f64) * t39679 + F::cast_from(0.18868855373762491241e-2_f64) * t39683;
    t41748
}
