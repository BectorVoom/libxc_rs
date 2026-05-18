//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1383/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1383<F: Float>(t2169: F, t233: F, t236: F, t27155: F, t27749: F, t27752: F, t2794: F, t5398: F, t7673: F, t8021: F, t8122: F, t911: F, t914: F, t91791: F, t91793: F, t91863: F, t91866: F, t91869: F, t91872: F, t95511: F, t97533: F) -> F {
    let t97547 = -t91791 - t91793 - t91863 + t91866 - t2794 * t8122 / F::new(8.0) - t91869 - t233 * t236 * (t95511 + t97533) / F::new(16.0) - t2169 * t914 * t5398 / F::new(8.0) - t27155 * t8021 / F::new(8.0) + t7673 * t27749 / F::new(8.0) + t91872 + t911 * t27752 / F::new(8.0);
    t97547
}
