//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1662/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1662<F: Float>(t3379: F, t5105: F, t12327: F, t1723: F, t3391: F, t12331: F, t3390: F, t5079: F, t1134: F, t3399: F, t5071: F, t3407: F) -> (F, F, F, F, F, F) {
    let t16846 = F::new(2.0) * t3379 * t5105;
    let t16851 = t12327 * t1723;
    let t16852 = t16851 * t3391;
    let t16854 = t12331 * t1723;
    let t16855 = t16854 * t3391;
    let t16857 = t3390 * t5079;
    let t16858 = t16857 * t1134;
    let t16860 = t5071 * t3399;
    let t16862 = t3407 * t5079;
    (t16846, t16852, t16855, t16858, t16860, t16862)
}
