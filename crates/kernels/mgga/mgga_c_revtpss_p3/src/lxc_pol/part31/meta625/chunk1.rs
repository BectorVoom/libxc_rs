//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2077/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2077<F: Float>(t25431: F, t99495: F, t14481: F, t1950: F, t2782: F, t2439: F, t7759: F, t780: F, t785: F, t25411: F, t2411: F, t27363: F) -> (F, F, F, F, F) {
    let t99496 = t25431 * t99495;
    let t99502 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t1950 * t14481;
    let t99520 = t2439 * t785 * t7759 * t780;
    let t99522 = t25411 * t99495;
    let t99555 = t27363 * t2411;
    (t99496, t99502, t99520, t99522, t99555)
}
