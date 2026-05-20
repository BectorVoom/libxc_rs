//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1379/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1379<F: Float>(t10638: F, t231: F, t243: F, t2661: F, t2662: F, t10722: F, t2656: F, t2237: F, t2482: F, t849: F, t2677: F, t10489: F, t221: F, t2674: F, t2675: F) -> (F, F, F, F) {
    let t40705 = t2661 * t2662 * t243 * t10638 * t231;
    let t40707 = t10722 * t2656;
    let t40710 = t2482 * t849 * t2237;
    let t40711 = t40710 * t2677;
    let t40719 = t2674 * t2675 * t221 * t10489;
    (t40705, t40707, t40711, t40719)
}
