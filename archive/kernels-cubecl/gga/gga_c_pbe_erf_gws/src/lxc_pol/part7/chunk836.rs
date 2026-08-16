//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 836/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk836<F: Float>(t184: F, t7631: F, t197: F, t5283: F, t1802: F, t1885: F, t1639: F, t649: F, t1642: F, t1: F, t837: F, t562: F, t577: F) -> (F, F, F, F, F, F, F) {
    let t7632 = t7631 * t184;
    let t7669 = t5283 * t197;
    let t7703 = t1885 * t1802;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7776 = t1 * t837;
    let t7838 = t562 * t577;
    (t7632, t7669, t7703, t7758, t7759, t7776, t7838)
}
