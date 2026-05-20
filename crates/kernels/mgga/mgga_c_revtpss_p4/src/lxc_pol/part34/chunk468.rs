//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 468/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk468<F: Float>(t2846: F, t1065: F, t159: F, t631: F) -> (F, F, F, F) {
    let t2847 = F::cast_from(0.23744444444444444444e-1_f64) * t2846;
    let t2850 = t159 * t1065;
    let t2851 = t631 * t631;
    let t2852 = F::new(1.0) / t2851;
    (t2847, t2850, t2851, t2852)
}
