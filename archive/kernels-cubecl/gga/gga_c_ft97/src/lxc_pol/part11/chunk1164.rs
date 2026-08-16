//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1164/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1164<F: Float>(t1526: F, t2640: F, t42262: F, t2666: F, t9483: F, t10227: F, t10215: F, t13598: F, t294: F, t9577: F, t10223: F, t10214: F, t10238: F, t10253: F, t15567: F, t18961: F, t18968: F, t2320: F, t3806: F, t9571: F, t9583: F, t9592: F) -> F {
    let t44663 = t1526 * t42262 * t2640;
    let t44666 = t1526 * t9483 * t2666;
    let t44669 = t1526 * t9483 * t10227;
    let t44672 = t1526 * t13598 * t10215;
    let t44674 = t294 * t9577;
    let t44683 = t1526 * t9483 * t10223;
    let t44685 = F::cast_from(2.0_f64) * t10238 + t1526 * t2320 * t10253 / F::cast_from(2.0_f64) - t1526 * t2320 * t10214 * t9571 / F::cast_from(2.0_f64) + t15567 * t18968 * t9592 / F::cast_from(2.0_f64) + t44663 / F::cast_from(18.0_f64) - t44666 / F::cast_from(6.0_f64) - t44669 / F::cast_from(12.0_f64) - t44672 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1526 * t3806 * t44674 * t9571 - t15567 * t18961 * t9583 / F::cast_from(3.0_f64) + t44683 / F::cast_from(6.0_f64);
    t44685
}
