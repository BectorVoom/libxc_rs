//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 784/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk784<F: Float>(t86: F, t32390: F, t32649: F, t113: F, t5: F, t505: F, t7293: F, t5764: F, t7150: F, t1374: F, t1774: F, t7298: F, t1360: F, t379: F) -> (F, F, F, F, F, F) {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t32650 = t32390 + t32649;
    let t32657 = piecewise3::<F>(t87, F::cast_from(0.0_f64), t5 * t32650 * t113 / F::cast_from(4.0_f64) + t5 * t7293 * t505 / F::cast_from(4.0_f64));
    let t32658 = t5764 * t7150;
    let t32661 = t1774 * t1374;
    let t32663 = t7298 * t32661 / F::cast_from(18.0_f64);
    let t32664 = t1360 * t379;
    (t32650, t32657, t32658, t32661, t32663, t32664)
}
