//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2075/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2075<F: Float>(t25875: F, t97703: F, t97705: F, t122: F, t3916: F, t72: F, t7910: F, t25895: F, t1398: F, t543: F, t5774: F, t1903: F, t4056: F) -> (F, F, F, F, F) {
    let t97719 = F::cast_from(0.25702851531048074406e-1_f64) * t25875 * t97703 * t97705;
    let t97732 = t7910 * t72 * t122 * t3916;
    let t97734 = F::cast_from(0.28912093960683998208e-1_f64) * t25895 * t97732;
    let t97737 = t5774 * t1398 * t543;
    let t97742 = t1903 * t4056 * t543;
    (t97719, t97732, t97734, t97737, t97742)
}
