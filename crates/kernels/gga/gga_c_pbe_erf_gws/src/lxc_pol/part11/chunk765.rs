//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 765/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk765<F: Float>(t12509: F, t625: F, t11: F, t12355: F, t626: F, t10581: F, t10583: F, t10585: F, t12495: F, t12499: F, t12503: F, t12507: F, t5360: F, t7269: F) -> (F, F, F, F, F, F) {
    let t12510 = t625 * t12509;
    let t12511 = t11 * t12510;
    let t12513 = t626 * t12355;
    let t12514 = t625 * t12513;
    let t12515 = t11 * t12514;
    let t12517 = t5360 + F::cast_from(0.25188888888888888889e-2_f64) * t7269 - F::cast_from(0.12594444444444444445e-2_f64) * t10581 + F::cast_from(0.37783333333333333335e-2_f64) * t10583 - F::cast_from(0.18891666666666666667e-2_f64) * t10585 + F::cast_from(0.20990740740740740742e-2_f64) * t12495 - F::cast_from(0.75566666666666666669e-2_f64) * t12499 + F::cast_from(0.37783333333333333335e-2_f64) * t12503 + F::new(0.11335e-1) * t12507 - F::new(0.11335e-1) * t12511 + F::cast_from(0.18891666666666666667e-2_f64) * t12515;
    (t12510, t12511, t12513, t12514, t12515, t12517)
}
