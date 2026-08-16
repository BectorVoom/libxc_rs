//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2095/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2095<F: Float>(t28019: F, t531: F, t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F, t1913: F, t7337: F, t116: F, t28042: F) -> (F, F, F, F, F, F) {
    let t101417 = t531 * t28019;
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    let t101454 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t101453;
    let t101455 = t25823 * t4287;
    let t101456 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101455;
    let t101563 = F::cast_from(2.0_f64) * t1913 * t7337;
    let t101622 = t116 * t28042;
    (t101417, t101451, t101454, t101456, t101563, t101622)
}
