//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1135/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1135<F: Float>(t4680: F, t7413: F, t9648: F, t1815: F, t1983: F, t30127: F, t7586: F, t31350: F, t6343: F, t30811: F, t6347: F, t142: F, t2060: F, t5674: F, t604: F) -> (F, F, F, F, F) {
    let t39643 = t7413 * t4680 * t9648;
    let t39647 = t30127 * t7586 * t1983 * t1815;
    let t39649 = t31350 * t6343;
    let t39653 = t30811 * t6347;
    let t39658 = t2060 * t142 * t604 * t5674;
    (t39643, t39647, t39649, t39653, t39658)
}
