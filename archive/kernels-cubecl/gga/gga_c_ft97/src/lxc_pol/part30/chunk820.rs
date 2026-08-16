//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 820/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk820<F: Float>(t299: F, t34259: F, t34336: F, t332: F, t5: F, t7691: F, t113: F, t505: F, t7692: F, t911: F, t1091: F, t33535: F, t2354: F) -> (F, F, F, F, F) {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t34337 = t34259 + t34336;
    let t34338 = t34337 * t332;
    let t34341 = t5 * t7691;
    let t34347 = piecewise3::<F>(t300, F::cast_from(0.0_f64), t5 * t34338 * t113 / F::cast_from(4.0_f64) + t5 * t7692 * t505 / F::cast_from(4.0_f64) + t34341 * t911 / F::cast_from(4.0_f64));
    let t35250 = t33535 * t1091;
    let t35251 = t2354 * t35250;
    (t34337, t34338, t34341, t34347, t35251)
}
