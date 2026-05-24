//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 796/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk796<F: Float>(t776: F, t11155: F, t11162: F, t12290: F, t12306: F, t1758: F, t1995: F, t4973: F, t4977: F, t525: F, t5449: F, t642: F, t7567: F, t773: F) -> F {
    let t777 = t776 < -F::new(0.66725e-1);
    let t12313 = piecewise3::<F>(t777, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t12290 * t642 - F::new(10.0) / F::new(9.0) * t525 * t5449 * t1758 + F::new(40.0) / F::new(27.0) * t525 * t1995 * t4973 - F::new(10.0) / F::new(9.0) * t525 * t1995 * t4977 - F::new(280.0) / F::new(243.0) * t525 * t773 * t11155 + F::new(40.0) / F::new(27.0) * t7567 * t12306 - F::new(10.0) / F::new(27.0) * t525 * t773 * t11162);
    t12313
}
