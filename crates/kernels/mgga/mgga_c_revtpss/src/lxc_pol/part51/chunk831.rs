//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 831/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk831<F: Float>(t1955: F, t4469: F, t72: F, t7778: F, t686: F, t7064: F, t1558: F, t231: F, t7048: F, t7076: F, t1949: F, t4423: F) -> (F, F, F, F, F, F) {
    let t27275 = t1955 * t4469;
    let t27278 = t7778 * t72;
    let t27279 = t27278 * t686;
    let t27280 = t7064 * t27279;
    let t27286 = t7048 * t1558 * t231;
    let t27287 = t7076 * t27286;
    let t27291 = t1949 * t4423 * t231;
    (t27275, t27279, t27280, t27286, t27287, t27291)
}
