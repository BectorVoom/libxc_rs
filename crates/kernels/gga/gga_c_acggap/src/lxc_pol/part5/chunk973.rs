//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 973/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk973<F: Float>(t1941: F, t3952: F, t11553: F, t103: F, t10952: F, t11545: F, t11549: F, t11552: F, t11557: F, t11560: F, t1674: F, t1679: F, t19289: F, t3984: F, t495: F, t5399: F, t5439: F, t560: F, t6583: F, t694: F, t811: F, t922: F, t96: F) -> (F, F) {
    let t19387 = t1941 * t3952;
    let t19394 = 0.24415263074675393405e-3 * t11553;
    let t19395 = -24.0 * t103 * t10952 * t3984 * t560 * t96 - 6.0 * t1674 * t6583 * t922 + 2.0 * t1679 * t19387 * t811 + 6.0 * t19289 * t495 * t694 - 12.0 * t5399 * t5439 * t694 + t11545 + t11549 - t11552 - t11557 - t11560 + t19394;
    (t19394, t19395)
}
