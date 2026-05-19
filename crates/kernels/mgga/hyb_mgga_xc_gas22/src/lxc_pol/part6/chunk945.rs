//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 945/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk945<F: Float>(t3444: F, t8604: F, t3443: F, t6737: F, t1347: F, t2228: F, t2188: F, t2189: F, t3356: F, t6579: F, t2236: F, t3352: F) -> (F, F, F, F, F, F, F) {
    let t8605 = t8604 * t3444;
    let t8608 = t3443 * t6737;
    let t8611 = t1347 * t2228;
    let t8613 = F::new(2.0) * t2188 * t8611;
    let t8614 = t3356 * t2189;
    let t8616 = F::cast_from(0.96491876992155210402e2_f64) * t6579 * t8614;
    let t8617 = t3352 * t2236;
    (t8605, t8608, t8611, t8613, t8614, t8616, t8617)
}
