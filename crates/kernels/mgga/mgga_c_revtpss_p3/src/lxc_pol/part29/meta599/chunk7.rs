//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2046/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2046<F: Float>(t2118: F, t5789: F, t1464: F, t8113: F, t1913: F, t7560: F, t2110: F, t5808: F, t1455: F, t8130: F, t1921: F, t7541: F) -> (F, F, F, F, F, F) {
    let t104071 = F::new(2.0) * t5789 * t2118;
    let t104073 = F::new(2.0) * t8113 * t1464;
    let t104077 = F::new(2.0) * t1913 * t7560;
    let t104079 = F::new(2.0) * t2110 * t5808;
    let t104081 = F::new(2.0) * t1455 * t8130;
    let t104083 = F::new(2.0) * t7541 * t1921;
    (t104071, t104073, t104077, t104079, t104081, t104083)
}
