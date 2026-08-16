//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1003/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1003<F: Float>(t10439: F, t162: F, t2516: F, t4398: F, t2496: F, t2619: F, t4302: F, t123: F, t1534: F) -> (F, F, F, F, F) {
    let t14330 = t10439 * t162;
    let t14334 = t4398 * t2516;
    let t14336 = t4398 * t2496;
    let t14339 = t4302 * t2619;
    let t14362 = t1534 * t123;
    (t14330, t14334, t14336, t14339, t14362)
}
