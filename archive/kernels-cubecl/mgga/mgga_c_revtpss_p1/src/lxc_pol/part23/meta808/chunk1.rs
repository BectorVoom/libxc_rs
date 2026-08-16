//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2643/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2643<F: Float>(t14991: F, t50208: F, t14485: F, t14987: F, t18657: F, t213: F, t14983: F, t18392: F, t262: F, t18838: F, t2411: F, t18969: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t63094 = t50208 * t14991;
    let t63099 = t14987 * t14485;
    let t63103 = t213 * t18657;
    let t63109 = t14987 * t14983;
    let t63146 = t262 * t18392;
    let t63160 = t18838 * t2411;
    let t63240 = t698 * t18969;
    (t63094, t63099, t63103, t63109, t63146, t63160, t63240)
}
