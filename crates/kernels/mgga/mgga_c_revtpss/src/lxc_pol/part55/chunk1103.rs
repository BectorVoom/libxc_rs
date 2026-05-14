//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1103/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1103<F: Float>(t122311: F, t27989: F, t122314: F, t125632: F, t125650: F, t122336: F, t122341: F, t125630: F, t125637: F, t125642: F, t125646: F, t1955: F, t2030: F, t28888: F, t122278: F, t27873: F) -> (F, F) {
    let t128652 = t122311 * t27989;
    let t128654 = t122314 * t27989;
    let t128656 = 0.1054086758983270768e-1 * t125632;
    let t128660 = 0.66119071333692697238e-4 * t125650;
    let t128664 = -0.14456046980341999104e-1 * t122336 + t122341 + 0.112937867033921868e-2 * t125630 + 0.28559868832551176308e-1 * t128652 - 0.50779446784275991476e-1 * t128654 + t128656 + 0.56468933516960933999e-3 * t125637 + 0.56468933516960933999e-3 * t125642 - 0.56468933516960933999e-3 * t125646 - t128660 - 0.8673628188205199462e0 * t1955 * t28888 * t2030;
    let t128665 = t122278 * t27873;
    (t128664, t128665)
}
