//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 642/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk642<F: Float>(t1564: F, t20145: F, t446: F, t1558: F, t20022: F, t356: F, t89: F, t20039: F, t447: F, t20044: F, t359: F, t11043: F, t15891: F, t15894: F, t20126: F, t20132: F, t20136: F, t20139: F, t20143: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20146 = t1564 * t20145;
    let t20147 = t446 * t20146;
    let t20149 = t1558 * t20022;
    let t20151 = t89 * t356 * t20149;
    let t20153 = t447 * t20039;
    let t20154 = t446 * t20153;
    let t20157 = t359 * t20044;
    let t20159 = t89 * t356 * t20157;
    let t20161 = -5.0 / 81.0 * t20126 + t15891 / 6.0 - t15894 / 3.0 + t20132 / 9.0 + 2.0 / 9.0 * t20136 - t20139 / 9.0 + t20143 / 6.0 + t20147 / 6.0 - t20151 / 3.0 + t20154 / 3.0 - 2.0 / 27.0 * t11043 - t20159 / 18.0;
    (t20146, t20147, t20149, t20151, t20153, t20154, t20157, t20159, t20161)
}
