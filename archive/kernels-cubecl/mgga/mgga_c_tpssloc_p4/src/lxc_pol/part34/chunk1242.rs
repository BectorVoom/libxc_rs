//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1242/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1242<F: Float>(t102137: F, t102139: F, t102142: F, t102173: F, t102267: F, t106836: F, t106862: F, t2032: F, t26911: F, t27966: F, t27972: F, t28935: F, t7432: F, t7435: F, t7782: F, t84216: F, t91905: F, t91922: F) -> F {
    let t108727 = -F::cast_from(2.0_f64) * t7435 * t28935 - F::cast_from(70.0_f64) * t84216 * t106836 - F::cast_from(5.0_f64) * t102267 * t7432 - F::cast_from(2.0_f64) * t106862 * t2032 - F::cast_from(10.0_f64) * t26911 * t27972 - F::cast_from(4.0_f64) * t27966 * t7782 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t102137 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t102139 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t102142 - F::cast_from(176.0_f64) / F::cast_from(9.0_f64) * t91905 - F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t91922 - F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t102173;
    t108727
}
