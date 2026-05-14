//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 817/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk817<F: Float>(t18440: F, t2662: F, t2661: F, t125: F, t6016: F, t2741: F, t5980: F, t5966: F, t2652: F, t5993: F, t6030: F, t10858: F, t6024: F, t6019: F, t10811: F, t6037: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18441 = t2662 * t18440;
    let t18442 = t2661 * t18441;
    let t18444 = t125 * t6016;
    let t18459 = t2741 * t5980;
    let t18469 = t125 * t5966;
    let t18475 = t2652 * t5993;
    let t18485 = t2652 * t6030;
    let t18487 = t10858 * t6024;
    let t18491 = t2741 * t6019;
    let t18518 = t10811 * t6037;
    (t18442, t18444, t18459, t18469, t18475, t18485, t18487, t18491, t18518)
}
