//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 778/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk778<F: Float>(t1642: F, t7758: F, t1: F, t837: F, t562: F, t577: F, t184: F, t219: F, t5400: F, t5480: F, t1563: F, t9: F, t155: F, t506: F, t1503: F, t522: F, t524: F) -> (F, F, F, F, F, F, F, F) {
    let t7759 = t7758 * t1642;
    let t7776 = t1 * t837;
    let t7838 = t562 * t577;
    let t7839 = t7838 * t184;
    let t7853 = t5400 * t219;
    let t7877 = t5480 * t219;
    let t8231 = t9 * t1563;
    let t8236 = t155 * t506;
    let t8331 = t1503 * t522 * t524;
    (t7759, t7776, t7839, t7853, t7877, t8231, t8236, t8331)
}
