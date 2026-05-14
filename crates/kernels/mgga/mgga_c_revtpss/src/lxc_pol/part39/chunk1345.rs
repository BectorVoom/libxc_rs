//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1345/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1345<F: Float>(t1913: F, t8302: F, t2192: F, t5789: F, t116890: F, t117095: F, t117369: F, t117374: F, t117720: F, t117765: F, t1458: F, t1464: F, t18178: F, t1921: F, t31088: F, t31329: F, t4154: F, t4168: F, t5790: F, t8373: F, t8389: F) -> (F,) {
    let t117772 = 2.0 * t1913 * t8302;
    let t117774 = 2.0 * t5789 * t2192;
    let t117777 = t117095 + t117369 + 2.0 * t5790 * t8302 + t4154 * t8389 + t117374 + t1458 * (t117720 + t117765) + t116890 + t18178 * t2192 + t8373 * t4168 + t31088 * t1921 + t117772 + t117774 + 2.0 * t31329 * t1464;
    (t117777,)
}
