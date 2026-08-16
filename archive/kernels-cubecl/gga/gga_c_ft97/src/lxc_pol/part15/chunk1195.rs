//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1195/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1195<F: Float>(t4917: F, t5309: F, t15195: F, t15460: F, t1901: F, t19500: F, t19506: F, t22187: F, t22245: F, t22368: F, t22405: F, t2862: F, t296: F, t319: F, t4139: F, t44280: F, t44566: F, t446: F, t5225: F, t5424: F, t55937: F, t84167: F, t84169: F, t84222: F, t90308: F, t90481: F) -> (F, F) {
    let t90717 = t4917 * t5309;
    let t90729 = F::cast_from(8.0_f64) * t446 * t44280 * t319 * t90308 + F::cast_from(4.0_f64) * t446 * t2862 * t5424 * t5225 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t84167 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t84169 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t55937 * t22368 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t19500 * t22187 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t84222 - t446 * t296 * t90481 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t4139 * t44566 * t90717 - F::cast_from(8.0_f64) * t1901 * t15460 * t19506 * t22405 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t15195 * t22245;
    (t90717, t90729)
}
