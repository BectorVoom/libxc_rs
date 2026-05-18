//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1265/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1265<F: Float>(t19113: F, t303: F, t7726: F, t2822: F, t28904: F, t27924: F, t4773: F, t6481: F, t7731: F, t1014: F, t28944: F, t27836: F, t2842: F, t4556: F) -> (F, F, F, F, F, F) {
    let t100678 = t303 * t7726 * t19113;
    let t100680 = t2822 * t28904;
    let t100683 = t303 * t27924 * t4773;
    let t100686 = t303 * t6481 * t7731;
    let t100688 = t1014 * t28944;
    let t100691 = t2842 * t27836 * t4556;
    (t100678, t100680, t100683, t100686, t100688, t100691)
}
