//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1218/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1218<F: Float>(t10298: F, t607: F, t2242: F, t2259: F, t1928: F, t25099: F, t25102: F, t25110: F, t25114: F, t25147: F, t25157: F, t25159: F, t6960: F, t6963: F, t6974: F, t92684: F, t92687: F, t92690: F, t92692: F, t92696: F, t92699: F, t92702: F) -> F {
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    let t92715 = t6963 * t25147 - F::new(15.0) * t92684 * t25159 - F::new(15.0) * t92687 * t25159 + F::new(35.0) * t92690 * t92692 - F::new(15.0) * t25157 * t92696 + F::new(5.0) / F::new(2.0) * t92699 * t6960 + F::new(5.0) * t92702 * t6960 + F::new(5.0) * t25099 * t25110 + F::new(5.0) / F::new(2.0) * t25099 * t25114 + t92709 * t1928 + t92711 * t1928 + F::new(2.0) * t25102 * t6974;
    t92715
}
