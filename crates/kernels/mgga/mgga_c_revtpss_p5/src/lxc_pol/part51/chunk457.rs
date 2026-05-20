//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 457/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk457<F: Float>(t1065: F, t159: F, t631: F, t2297: F, t910: F, t914: F, t287: F, t913: F, t275: F, t273: F, t276: F, t2846: F) -> (F, F, F, F, F, F, F) {
    let t2850 = t159 * t1065;
    let t2851 = t631 * t631;
    let t2852 = F::new(1.0) / t2851;
    let t2857 = F::new(1.0) / t2297;
    let t2869 = t910 * t914;
    let t2872 = t913 * t287;
    let t2873 = F::new(1.0) / t2872;
    let t2874 = t275 * t2873;
    let t2880 = F::new(1.0) / t276 / t273;
    let t2884 = F::new(4.0) / F::new(9.0) * t2846;
    (t2850, t2852, t2857, t2869, t2874, t2880, t2884)
}
