//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1113/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1113<F: Float>(t120967: F, t125627: F, t247: F, t3938: F, t125609: F, t31805: F, t32240: F, t33946: F, t686: F, t72: F, t121365: F, t33951: F, t689: F) -> (F, F, F, F, F) {
    let t125677 = t120967 * t247 * t125627 * t3938;
    let t125680 = t31805 * t125609;
    let t125681 = t125680 * t32240;
    let t125690 = t33946 * t72 * t686;
    let t125691 = t121365 * t125690;
    let t125693 = t33951 * t689;
    (t125677, t125681, t125690, t125691, t125693)
}
