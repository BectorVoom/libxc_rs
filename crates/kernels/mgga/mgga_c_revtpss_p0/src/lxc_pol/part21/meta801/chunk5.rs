//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2912/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2912<F: Float>(t300: F, t52282: F, t52324: F, t52377: F, t52433: F, t52477: F, t52520: F, t52817: F, t52856: F, t11502: F, t4724: F, t981: F) -> (F, F) {
    let t52860 = t300 * (t52282 + t52324 + t52377 + t52433 + t52477 + t52520 + t52817 + t52856);
    let t52863 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t4724 * t11502;
    (t52860, t52863)
}
