//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1659/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1659<F: Float>(t291: F, t88100: F, t88137: F, t141: F, t41294: F, t88102: F, t88106: F, t930: F, t11341: F, t88112: F, t2908: F, t88120: F) -> (F, F, F, F, F) {
    let t88140 = F::cast_from(0.621814e-1_f64) * (t88100 + t88137) * t291;
    let t88144 = t141 * t41294 * t88102;
    let t88147 = t141 * t930 * t88106;
    let t88150 = t141 * t11341 * t88112;
    let t88161 = t141 * t2908 * t88120;
    (t88140, t88144, t88147, t88150, t88161)
}
