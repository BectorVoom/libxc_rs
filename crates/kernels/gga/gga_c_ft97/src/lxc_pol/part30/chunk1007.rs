//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1007/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1007<F: Float>(t140769: F, t140784: F, t140795: F, t140797: F, t150154: F, t150158: F, t150162: F, t150165: F, t150168: F, t150171: F, t150175: F, t150179: F, t150184: F, t150188: F, t150194: F, t150199: F) -> F {
    let t150201 = -F::cast_from(6.0_f64) * t150154 - F::cast_from(12.0_f64) * t150158 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t150162 + t150165 / F::cast_from(6.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t150168 - t150171 / F::cast_from(3.0_f64) - t150175 / F::cast_from(12.0_f64) - t150179 / F::cast_from(12.0_f64) - F::cast_from(20.0_f64) * t150184 + F::cast_from(8.0_f64) * t150188 - t140769 + t140784 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140795 - t140797 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t150194 + t150199 / F::cast_from(4.0_f64);
    t150201
}
