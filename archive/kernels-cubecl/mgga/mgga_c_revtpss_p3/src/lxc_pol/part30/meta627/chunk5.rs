//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2179/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2179<F: Float>(t2435: F, t7774: F, t25431: F, t14481: F, t1950: F, t2782: F, t4424: F, t886: F, t2439: F, t7759: F, t780: F, t785: F) -> (F, F, F, F, F) {
    let t99495 = t7774 * t2435;
    let t99496 = t25431 * t99495;
    let t99502 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t1950 * t14481;
    let t99512 = t4424 * t886;
    let t99520 = t2439 * t785 * t7759 * t780;
    (t99495, t99496, t99502, t99512, t99520)
}
