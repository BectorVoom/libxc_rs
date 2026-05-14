//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1184/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1184<F: Float>(t3924: F, t676: F, t25880: F, t25899: F, t10008: F, t1955: F, t2022: F, t9646: F, t9648: F, t25875: F, t94394: F, t94398: F, t46361: F, t545: F, t9685: F, t25895: F) -> (F, F, F, F, F, F, F, F) {
    let t94639 = t676 * t3924;
    let t94640 = t25880 * t94639;
    let t94641 = t25899 * t94640;
    let t94643 = t1955 * t10008;
    let t94648 = 0.19637199382202157274e-3 * t9646 * t2022 * t9648;
    let t94649 = t25875 * t94394;
    let t94650 = t94649 * t94398;
    let t94656 = t46361 * t545;
    let t94661 = t25880 * t9685;
    let t94662 = t25895 * t94661;
    (t94640, t94641, t94643, t94648, t94650, t94656, t94661, t94662)
}
