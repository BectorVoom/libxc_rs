//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1024/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1024<F: Float>(t119808: F, t31805: F, t31801: F, t2470: F, t31800: F, t31806: F, t2670: F, t31827: F, t31809: F, t31845: F, t11007: F, t3140: F) -> (F, F, F, F, F, F) {
    let t119809 = t31805 * t119808;
    let t119810 = t119809 * t31801;
    let t119813 = t31800 * t2470;
    let t119815 = F::new(0.33852964522850660984e-1) * t31806 * t119813;
    let t119816 = t31827 * t2670;
    let t119817 = F::new(0.19833242244827205771e-3) * t119816;
    let t119818 = t31809 * t31845;
    let t119821 = t3140 * t11007;
    (t119810, t119813, t119815, t119817, t119818, t119821)
}
