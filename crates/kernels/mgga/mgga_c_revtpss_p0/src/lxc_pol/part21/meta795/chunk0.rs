//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2875/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2875<F: Float>(t15397: F, t41583: F, t2874: F, t2918: F, t4632: F, t15534: F, t3022: F, t1100: F, t3329: F, t15537: F, t3007: F, t981: F) -> (F, F, F, F, F) {
    let t52182 = F::cast_from(0.1551780387578202009e4_f64) * t41583 * t15397;
    let t52185 = F::new(6.0) * t2874 * t4632 * t2918;
    let t52187 = F::cast_from(0.17544670867903938621e1_f64) * t3022 * t15534;
    let t52188 = t1100 * t3329;
    let t52194 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t15537 * t3007;
    (t52182, t52185, t52187, t52188, t52194)
}
