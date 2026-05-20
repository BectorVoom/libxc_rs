//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2941/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2941<F: Float>(t77998: F, t78010: F, t78023: F, t78035: F, t78049: F, t78061: F, t78075: F, t78088: F, t915: F, t935: F, t23550: F, t41583: F) -> (F, F) {
    let t78094 = F::new(1.0) * t915 * (t77998 + t78010 + t78023 + t78035 + t78049 + t78061 + t78075 + t78088) * t935;
    let t78096 = F::cast_from(0.51726012919273400301e3_f64) * t41583 * t23550;
    (t78094, t78096)
}
