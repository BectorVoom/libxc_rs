//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1133/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1133<F: Float>(t3827: F, t3856: F, t3859: F, t3862: F, t3865: F, t3867: F, t4031: F, t4033: F, t4035: F, t4037: F, t4040: F, t4042: F) -> F {
    let t4043 = t3856 + t4031 - t4033 - t3867 - t4035 - t4037 - t4040 + t3859 + t3862 - t3865 - t3827 + t4042;
    t4043
}
