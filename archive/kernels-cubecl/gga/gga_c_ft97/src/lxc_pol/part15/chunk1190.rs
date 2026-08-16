//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1190/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1190<F: Float>(t845: F, t90359: F, t90379: F, t90421: F, t90464: F, t91: F, t44121: F, t71276: F, t71298: F, t71305: F, t71319: F, t83728: F, t83770: F, t83772: F, t83781: F, t83789: F, t83792: F, t90326: F, t90330: F, t90335: F) -> (F, F) {
    let t90468 = t91 * t845 * (t90359 + t90379 + t90421 + t90464);
    let t90478 = F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t83728 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t90326 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t90330 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t71276 + t44121 + F::cast_from(8.0_f64) * t90335 + t90468 / F::cast_from(2.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t83770 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t83772 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t83781 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t83789 + F::cast_from(8.0_f64) * t83792 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t71298 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t71305 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t71319;
    (t90468, t90478)
}
