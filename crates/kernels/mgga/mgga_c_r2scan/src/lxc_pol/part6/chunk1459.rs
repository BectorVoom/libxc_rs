//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1459/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1459<F: Float>(t19694: F, t19698: F, t19748: F, t23964: F, t23968: F, t23970: F, t23972: F, t23980: F, t23982: F, t23984: F, t23986: F, t19702: F, t19709: F, t23990: F, t23992: F, t25032: F, t25034: F, t25037: F, t27375: F, t27386: F, t27393: F, t8306: F, t860: F) -> (F, F) {
    let t27450 = t23964 + t23968 + t23970 - t23972 - t19748 - t23980 + t23982 - t23984 - t19694 + t19698 + t23986;
    let t27454 = 3.0 * t8306 * t860 + t19702 + t19709 + t23990 - t23992 - t25032 + t25034 + t25037 + t27375 + t27386 - t27393;
    (t27450, t27454)
}
