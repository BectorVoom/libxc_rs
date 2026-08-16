//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 690/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk690<F: Float>(t20153: F, t446: F, t20044: F, t359: F, t356: F, t89: F, t11043: F, t15891: F, t15894: F, t20126: F, t20132: F, t20136: F, t20139: F, t20143: F, t20147: F, t20151: F) -> (F, F, F, F) {
    let t20154 = t446 * t20153;
    let t20157 = t359 * t20044;
    let t20159 = t89 * t356 * t20157;
    let t20161 = -F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t20126 + t15891 / F::cast_from(6.0_f64) - t15894 / F::cast_from(3.0_f64) + t20132 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t20136 - t20139 / F::cast_from(9.0_f64) + t20143 / F::cast_from(6.0_f64) + t20147 / F::cast_from(6.0_f64) - t20151 / F::cast_from(3.0_f64) + t20154 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11043 - t20159 / F::cast_from(18.0_f64);
    (t20154, t20157, t20159, t20161)
}
