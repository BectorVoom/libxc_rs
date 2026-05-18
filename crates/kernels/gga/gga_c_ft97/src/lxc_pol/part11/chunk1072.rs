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
    let t42250 = F::new(4.0) / F::new(9.0) * t41947 + t42044 - F::new(5.0) / F::new(16.0) * t42053 - t42057 / F::new(4.0) + t42233 / F::new(6.0) - t42236 / F::new(3.0) + F::new(3.0) / F::new(4.0) * t42240 - F::new(8.0) / F::new(27.0) * t41953 - F::new(16.0) / F::new(81.0) * t41957 - F::new(16.0) / F::new(27.0) * t41960 + F::new(40.0) / F::new(243.0) * t41964 + F::new(40.0) / F::new(27.0) * t41969 - t41973 / F::new(9.0) - F::new(12.0) * t41978 + F::new(112.0) / F::new(243.0) * t41981;
    (t42233, t42236, t42240, t42250)
}
