//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1082/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1082<F: Float>(t1658: F, t2169: F, t233: F, t28300: F, t29223: F, t29229: F, t6883: F, t7673: F, t914: F, t91791: F, t91793: F, t91863: F, t91866: F, t91869: F, t91872: F, t91874: F) -> (F,) {
    let t99810 = -t91791 - t91793 - t91863 + t7673 * t29229 / 8.0 + t91866 - t91869 + t91872 - t91874 - t2169 * t914 * t6883 / 16.0 - t233 * t1658 * t28300 / 8.0 + t7673 * t29223 / 16.0;
    (t99810,)
}
