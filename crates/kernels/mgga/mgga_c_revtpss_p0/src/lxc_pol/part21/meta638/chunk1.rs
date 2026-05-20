//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2413/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2413<F: Float>(t2439: F, t2440: F, t2829: F, t10977: F, t2465: F, t686: F, t72: F, t11061: F, t11064: F, t2410: F, t2832: F, t775: F) -> (F, F, F, F, F) {
    let t41125 = t2439 * t2440 * t2829;
    let t41129 = t2465 * t10977 * t72 * t686;
    let t41137 = t11061 * t11064;
    let t41153 = t2410 * t2410;
    let t41154 = F::new(1.0) / t41153;
    let t41161 = t775 * t2832;
    (t41125, t41129, t41137, t41154, t41161)
}
