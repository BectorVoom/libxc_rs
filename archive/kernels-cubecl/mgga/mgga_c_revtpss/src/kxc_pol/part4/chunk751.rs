//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 751/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk751<F: Float>(t136: F, t561: F, t2457: F, t3906: F, t1420: F, t786: F, t1364: F, t1426: F, t556: F) -> (F, F, F, F, F, F, F) {
    let t3907 = t561 * t136;
    let t3908 = t3907 * t2457;
    let t3910 = F::cast_from(0.11565819519348392139e-2_f64) * t3906 * t3908;
    let t3911 = t786 * t1420;
    let t3912 = t3911 * t1364;
    let t3914 = t556 * t1426;
    let t3915 = t786 * t3914;
    (t3907, t3908, t3910, t3911, t3912, t3914, t3915)
}
