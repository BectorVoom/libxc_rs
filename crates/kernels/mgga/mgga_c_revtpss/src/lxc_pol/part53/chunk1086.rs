//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1086/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1086<F: Float>(t119813: F, t31806: F, t2670: F, t31827: F, t31809: F, t31845: F, t11007: F, t3140: F, t822: F, t31830: F, t122: F, t72: F, t8471: F) -> (F, F, F, F, F, F, F) {
    let t119815 = F::new(0.33852964522850660984e-1) * t31806 * t119813;
    let t119816 = t31827 * t2670;
    let t119817 = F::new(0.19833242244827205771e-3) * t119816;
    let t119818 = t31809 * t31845;
    let t119821 = t3140 * t11007;
    let t119822 = t119821 * t822;
    let t119823 = t31830 * t119822;
    let t119825 = t8471 * t72 * t122;
    (t119815, t119817, t119818, t119821, t119822, t119823, t119825)
}
