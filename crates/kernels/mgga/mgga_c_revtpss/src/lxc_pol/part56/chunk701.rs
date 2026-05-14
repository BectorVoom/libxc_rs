//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 701/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk701<F: Float>(t88: F, t89: F, t90: F, t29: F, t2475: F, t72: F, t245: F, t136: F, t853: F, t220: F, t821: F, t866: F, t2410: F, t261: F) -> (F, F, F, F, F, F, F, F) {
    let t10308 = 1.0 / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10769 = t2475 * t72;
    let t10770 = t10769 * t245;
    let t10778 = t853 * t136;
    let t10779 = t10778 * t220;
    let t10866 = t821 * t821;
    let t10867 = 1.0 / t10866;
    let t11006 = t866 * t866;
    let t11007 = 1.0 / t11006;
    let t11064 = 1.0 / t2410 / t261;
    (t10308, t10309, t10770, t10779, t10867, t11006, t11007, t11064)
}
