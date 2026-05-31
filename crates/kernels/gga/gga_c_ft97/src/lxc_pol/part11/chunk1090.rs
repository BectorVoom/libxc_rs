//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1090/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1090<F: Float>(t41950: F, t41947: F, t41953: F, t41957: F, t41960: F, t41964: F, t41969: F, t41973: F, t41978: F, t41981: F, t42053: F, t42057: F, t42233: F, t42236: F, t42240: F) -> F {
    let t42759 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t41950;
    let t42772 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t41947 + t42759 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t42053 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42057 + t42233 / F::cast_from(2.0_f64) - t42236 + F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t42240 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41953 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t41957 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t41960 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t41964 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t41969 - t41973 / F::cast_from(3.0_f64) - F::cast_from(36.0_f64) * t41978 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t41981;
    t42772
}
