//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1028/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1028<F: Float>(t119868: F, t25304: F, t8464: F, t233: F, t240: F, t31752: F, t843: F, t31774: F, t31769: F, t41077: F, t822: F, t119858: F, t7063: F) -> (F, F, F, F, F, F) {
    let t119869 = t25304 * t8464 * t119868;
    let t119870 = F::new(0.17851433602423232928e-4) * t119869;
    let t119875 = t31752 * t233 * t843 * t240;
    let t119876 = t119875 * t31774;
    let t119878 = t119875 * t31769;
    let t119883 = t41077 * t822;
    let t119888 = t7063 * t119858;
    (t119870, t119875, t119876, t119878, t119883, t119888)
}
