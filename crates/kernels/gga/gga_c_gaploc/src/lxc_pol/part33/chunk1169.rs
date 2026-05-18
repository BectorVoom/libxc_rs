//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1169/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1169<F: Float>(t31694: F, t1365: F, t25740: F, t6525: F, t2268: F, t2349: F, t7930: F, t2765: F, t6474: F, t23741: F, t3327: F, t10113: F, t6305: F) -> (F, F, F, F, F, F) {
    let t31695 = F::new(0.23712505529730124666e-2) * t31694;
    let t31697 = t6525 * t1365 * t25740;
    let t31698 = F::new(0.11856252764865062333e-2) * t31697;
    let t31701 = F::new(0.17073003981405689759e0) * t2268 * t7930 * t2349;
    let t31704 = F::new(0.85365019907028448797e-1) * t2268 * t2765 * t6474;
    let t31706 = F::new(0.28455006635676149599e-1) * t23741 * t3327;
    let t31708 = F::new(0.56910013271352299198e-1) * t6305 * t10113;
    (t31695, t31698, t31701, t31704, t31706, t31708)
}
