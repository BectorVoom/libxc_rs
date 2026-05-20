//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1935/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1935<F: Float>(t14468: F, t33: F, t25759: F, t61102: F, t61182: F, t27799: F, t98779: F, t1711: F, t2394: F, t2430: F, t27375: F, t94245: F) -> (F, F, F, F, F, F, F) {
    let t101051 = t33 * t14468;
    let t101055 = t25759 * t61102;
    let t101061 = t25759 * t61182;
    let t101065 = t27799 * t98779;
    let t101070 = t1711 * t2394;
    let t101074 = t1711 * t2430;
    let t101083 = t94245 * t27375;
    (t101051, t101055, t101061, t101065, t101070, t101074, t101083)
}
