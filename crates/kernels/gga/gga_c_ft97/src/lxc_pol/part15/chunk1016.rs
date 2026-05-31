//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1016/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1016<F: Float>(t10969: F, t20268: F, t20213: F, t2983: F, t11490: F, t11810: F, t11811: F, t11902: F, t11906: F, t16145: F, t1901: F, t1902: F, t20113: F, t20191: F, t20214: F, t20219: F, t20438: F, t446: F, t4495: F, t452: F, t4623: F, t47273: F, t60309: F, t75370: F, t75372: F, t75678: F, t8411: F, t925: F, t986: F) -> (F, F, F) {
    let t85797 = t10969 * t20268;
    let t85825 = t2983 * t20213;
    let t85862 = -F::cast_from(8.0_f64) * t1901 * t11490 * t16145 * t20268 - F::cast_from(8.0_f64) * t1901 * t11810 * t11811 * t20191 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t47273 * t20438 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t60309 - F::cast_from(8.0_f64) * t446 * t8411 * t986 * t20113 - F::cast_from(2.0_f64) * t446 * t452 * t4623 * t4495 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t1902 * t75678 * t925 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11902 * t20214 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11906 * t20219 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t75370 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t75372;
    (t85797, t85825, t85862)
}
