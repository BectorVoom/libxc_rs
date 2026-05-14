//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1218/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1218<F: Float>(t93826: F, t1295: F, t15793: F, t2169: F, t233: F, t235: F, t27743: F, t27755: F, t27758: F, t28300: F, t4533: F, t7673: F, t911: F, t915: F, t92157: F, t92165: F, t92168: F, t92170: F, t92339: F, t93817: F) -> (F,) {
    let t97584 = 2.0 * t93826;
    let t97585 = t92157 + t7673 * t27758 / 8.0 - t233 * t915 * t28300 / 8.0 + t911 * t27743 / 8.0 + t93817 + t911 * t27755 / 8.0 - t92165 - t2169 * t235 * t15793 / 16.0 - t2169 * t4533 * t1295 / 8.0 + t97584 + t92168 + t92170 + t92339;
    (t97585,)
}
