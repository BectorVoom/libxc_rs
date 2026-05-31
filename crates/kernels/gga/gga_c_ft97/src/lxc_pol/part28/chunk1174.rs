//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1174/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1174<F: Float>(t1882: F, t35035: F, t11593: F, t12714: F, t140395: F, t140397: F, t140412: F, t140424: F, t140426: F, t1557: F, t1901: F, t26955: F, t26999: F, t3052: F, t3188: F, t3281: F, t33039: F, t3450: F, t446: F, t569: F, t574: F, t5935: F, t7357: F, t7407: F, t7414: F, t9144: F) -> F {
    let t149256 = t1882 * t35035;
    let t149263 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t12714 * t7407 * t1557 * t3188 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t9144 * t7357 * t3052 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t569 * t7414 * t3052 + t140395 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140397 - t140412 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t5935 * t26955 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140424 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t149256 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t140426 - F::cast_from(2.0_f64) * t1901 * t26999 * t33039 * t3450;
    t149263
}
