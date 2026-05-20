//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1850/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1850<F: Float>(t94564: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t25877: F, t94390: F, t1032: F, t4066: F, t1955: F) -> (F, F, F, F, F, F) {
    let t94565 = t94564 * t9795;
    let t94568 = t40688 * t2018 * t46808;
    let t94570 = t9784 * t7256;
    let t94589 = t94390 * t25877;
    let t94609 = t4066 * t1032;
    let t94610 = t1955 * t94609;
    (t94565, t94568, t94570, t94589, t94609, t94610)
}
