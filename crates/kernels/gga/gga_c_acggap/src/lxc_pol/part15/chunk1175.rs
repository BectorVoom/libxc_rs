//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1175/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1175<F: Float>(t1524: F, t2060: F, t2288: F, t8927: F, t2030: F, t35413: F, t5697: F, t34903: F, t5693: F, t7450: F, t372: F, t4262: F, t9529: F) -> (F, F, F, F) {
    let t40344 = t2060 * t8927 * t2288 * t1524;
    let t40347 = t2030 * t35413 * t5697;
    let t40350 = t7450 * t34903 * t5693;
    let t40354 = t7450 * t4262 * t9529 * t372;
    (t40344, t40347, t40350, t40354)
}
