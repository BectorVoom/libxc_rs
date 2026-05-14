//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1266/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1266<F: Float>(t3491: F, t5165: F, t135: F, t144: F, t16526: F, t16531: F, t16536: F, t16539: F, t16544: F, t1673: F, t23940: F, t23941: F, t23943: F, t23944: F, t23992: F, t24757: F, t24922: F, t2536: F, t560: F, t639: F) -> (F,) {
    let t24927 = t3491 * t5165;
    let t24931 = t23940 - t23941 + t16526 + t23943 + t16531 + t23944 + t16536 - t16539 + 3.0 * t135 * t560 * t23992 + t135 * t144 * (t24757 + t24922) * t639 + 2.0 * t2536 * t24927 * t1673 - t16544;
    (t24931,)
}
