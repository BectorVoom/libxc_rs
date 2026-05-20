//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1792/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1792<F: Float>(t4144: F, t4146: F, t198: F, t25177: F, t39989: F, t4135: F, t4139: F, t4140: F, t47076: F, t47079: F, t47082: F, t47084: F, t47086: F, t47088: F, t47090: F, t47092: F, t47094: F, t47096: F, t47098: F, t532: F, t5541: F, t9628: F) -> F {
    let t47669 = t4144 * t4144;
    let t47671 = t4146 * t4146;
    let t47672 = F::new(1.0) / t47671;
    let t47676 = -F::new(6.0) * t198 * t47669 * t47672 * t532 + F::new(12.0) * t25177 * t4135 * t5541 + F::new(12.0) * t4139 * t4140 * t9628 - t39989 - t47076 - t47079 + t47082 - t47084 - t47086 + t47088 + t47090 + t47092 + t47094 - t47096 - t47098;
    t47676
}
