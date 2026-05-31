//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1353/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1353<F: Float>(t10541: F, t8601: F, t1049: F, t30867: F, t11046: F, t9378: F, t15436: F, t3832: F, t1616: F, t3179: F, t3537: F, t12055: F, t4908: F) -> (F, F, F, F, F, F) {
    let t36303 = F::cast_from(4.0_f64) * t8601 * t10541;
    let t36304 = t30867 * t1049;
    let t36307 = F::cast_from(4.0_f64) * t11046 * t9378;
    let t36309 = F::cast_from(2.0_f64) * t15436 * t3832;
    let t36312 = F::cast_from(4.0_f64) * t1616 * t3537 * t3179;
    let t36314 = F::cast_from(4.0_f64) * t4908 * t12055;
    (t36303, t36304, t36307, t36309, t36312, t36314)
}
