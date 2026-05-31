//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1147/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1147<F: Float>(t1168: F, t21639: F, t2568: F, t10024: F, t1175: F, t18391: F, t21351: F, t2594: F, t265: F, t42469: F, t446: F, t4973: F, t5053: F, t5064: F, t5073: F, t5181: F, t68200: F, t724: F, t729: F, t88726: F, t88730: F, t88742: F, t88749: F, t88756: F, t88764: F) -> (F, F) {
    let t89273 = t2568 * t1168 * t21639;
    let t89343 = -F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t446 * t10024 * t1175 * t21351 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t446 * t42469 * t265 * t88726 - t446 * t724 * t265 * t88730 / F::cast_from(9.0_f64) + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68200 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t724 * t5181 * t4973 - F::cast_from(4.0_f64) * t446 * t729 * t2568 * t5053 * t5064 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t724 * t265 * t88749 + F::cast_from(4.0_f64) * t446 * t729 * t18391 * t5073 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t446 * t10024 * t265 * t88764 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t2594 * t265 * t88742 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t724 * t265 * t88756;
    (t89273, t89343)
}
