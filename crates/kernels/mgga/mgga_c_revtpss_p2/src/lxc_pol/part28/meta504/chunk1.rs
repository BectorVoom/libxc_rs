//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1895/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1895<F: Float>(t27137: F, t651: F, t1843: F, t1932: F, t2322: F, t27116: F, t27118: F, t27120: F, t27122: F, t27125: F, t27128: F, t27130: F, t27132: F, t27134: F, t27136: F, t5517: F, t6983: F, t7746: F) -> F {
    let t27139 = F::new(2.0) * t651 * t27137;
    let t27142 = -t1843 * t6983 - t1932 * t5517 - F::new(2.0) * t2322 * t7746 - t27116 - t27118 - t27120 - t27122 - t27125 - t27128 - t27130 - t27132 - t27134 - t27136 - t27139;
    t27142
}
