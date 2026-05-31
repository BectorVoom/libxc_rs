//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1278/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1278<F: Float>(t11880: F, t7111: F, t11752: F, t11755: F, t1017: F, t11759: F, t11811: F, t11824: F, t25539: F, t3248: F, t3255: F, t7117: F, t93683: F, t93685: F, t93687: F, t93689: F, t93691: F, t93694: F) -> F {
    let t93696 = t7111 * t11880;
    let t93702 = t7111 * t11752;
    let t93704 = t7111 * t11755;
    let t93710 = -F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t11811 - F::cast_from(0.17149607247227894789e-2_f64) * t93683 - F::cast_from(0.85748036236139473944e-3_f64) * t93685 - F::cast_from(0.11433071498151929859e-2_f64) * t93687 + F::cast_from(0.17149607247227894789e-2_f64) * t93689 + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t93691 * t1017 - t93694 / F::cast_from(54.0_f64) - t93696 / F::cast_from(432.0_f64) - t25539 * t3248 / F::cast_from(36.0_f64) - t25539 * t3255 / F::cast_from(27.0_f64) + t93702 / F::cast_from(288.0_f64) + t93704 / F::cast_from(216.0_f64) + t7111 * t11759 / F::cast_from(288.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t7111 * t11824;
    t93710
}
