//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 938/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk938<F: Float>(t33474: F, t33508: F, t8937: F, t96886: F, t1276: F, t13038: F, t3598: F, t33516: F, t127: F, t33495: F, t33496: F, t371: F, t44841: F, t8936: F, t33494: F, t487: F) -> (F, F, F, F, F, F) {
    let t124554 = t33474 * t33508;
    let t124557 = t8937 * t96886;
    let t124560 = t124557 * t1276 * t13038 * t3598;
    let t124564 = t124557 * t33516 * t3598;
    let t124569 = t33495 * t371 * t127 * t33496;
    let t124571 = t8936 * t44841;
    let t124573 = t124571 * t487 * t33494;
    (t124554, t124560, t124564, t124569, t124571, t124573)
}
