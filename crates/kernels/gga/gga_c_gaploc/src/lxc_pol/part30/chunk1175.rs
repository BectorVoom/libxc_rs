//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1175/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1175<F: Float>(t1222: F, t3344: F, t10262: F, t484: F, t1217: F, t3351: F, t2317: F, t6525: F, t7901: F, t1365: F, t25740: F, t2268: F, t2349: F, t7930: F) -> (F, F, F, F, F, F) {
    let t31687 = t1222 * t3344;
    let t31688 = F::new(0.31616674039640166222e-2) * t31687;
    let t31689 = t484 * t10262;
    let t31690 = F::new(0.31616674039640166222e-2) * t31689;
    let t31691 = t1217 * t3351;
    let t31692 = F::new(0.36886119712913527259e-2) * t31691;
    let t31694 = t6525 * t7901 * t2317;
    let t31695 = F::new(0.23712505529730124666e-2) * t31694;
    let t31697 = t6525 * t1365 * t25740;
    let t31698 = F::new(0.11856252764865062333e-2) * t31697;
    let t31701 = F::new(0.17073003981405689759e0) * t2268 * t7930 * t2349;
    (t31688, t31690, t31692, t31695, t31698, t31701)
}
