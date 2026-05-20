//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1809/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1809<F: Float>(t18534: F, t18553: F, t18568: F, t18583: F, t225: F, t1553: F, t73: F, t2475: F, t5966: F) -> (F, F, F) {
    let t18586 = (t18534 + t18553 + t18568 + t18583) * t225;
    let t18592 = t1553 * t73;
    let t18599 = t2475 * t5966;
    (t18586, t18592, t18599)
}
