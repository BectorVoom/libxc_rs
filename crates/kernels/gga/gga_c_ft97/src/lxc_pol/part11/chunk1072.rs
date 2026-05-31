//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1072/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1072<F: Float>(t42104: F, t42143: F, t42191: F, t42229: F, t734: F, t91: F, t9881: F, t9968: F, t2476: F, t2514: F, t9890: F, t41947: F, t41953: F, t41957: F, t41960: F, t41964: F, t41969: F, t41973: F, t41978: F, t41981: F, t42044: F, t42053: F, t42057: F) -> (F, F, F, F) {
    let t42233 = t91 * t734 * (t42104 + t42143 + t42191 + t42229);
    let t42236 = t91 * t9881 * t9968;
    let t42240 = t91 * t9890 * t2476 * t2514;
    let t42250 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41947 + t42044 - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t42053 - t42057 / F::cast_from(4.0_f64) + t42233 / F::cast_from(6.0_f64) - t42236 / F::cast_from(3.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42240 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t41953 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t41957 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t41960 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t41964 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t41969 - t41973 / F::cast_from(9.0_f64) - F::cast_from(12.0_f64) * t41978 + F::cast_from(112.0_f64) / F::cast_from(243.0_f64) * t41981;
    (t42233, t42236, t42240, t42250)
}
