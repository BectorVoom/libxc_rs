//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2091/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2091<F: Float>(t100690: F, t994: F, t7150: F, t7810: F, t989: F, t25698: F, t27418: F, t4746: F, t7135: F, t1982: F, t99708: F, t3047: F, t8521: F) -> (F, F, F, F, F, F, F) {
    let t100691 = t994 * t100690;
    let t100698 = t7150 * t100690;
    let t100702 = t989 * t7810;
    let t100705 = t25698 * t27418;
    let t100708 = t4746 * t7135;
    let t100723 = t1982 * t99708;
    let t100737 = t3047 * t8521;
    (t100691, t100698, t100702, t100705, t100708, t100723, t100737)
}
