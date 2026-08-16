//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1216/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1216<F: Float>(t10327: F, t603: F, t1928: F, t25106: F, t25114: F, t25120: F, t25140: F, t25143: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F, t92654: F, t92658: F, t92662: F, t92666: F, t92669: F, t92672: F) -> F {
    let t92674 = t603 * t10327;
    let t92682 = F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t25106 * t25114 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t6958 * t92654 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t6958 * t92658 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t92662 - F::cast_from(5.0_f64) * t92666 * t6960 + t603 * t92669 * t92672 + t92674 * t1928 / F::cast_from(3.0_f64) + t25120 * t6974 + t25120 * t6978 + t6963 * t25140 + F::cast_from(2.0_f64) * t6963 * t25143;
    t92682
}
