//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1049/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1049<F: Float>(t149920: F, t193: F, t713: F, t89: F, t141340: F, t141364: F, t141368: F, t150953: F, t150958: F, t150962: F, t150966: F, t150971: F, t150974: F, t150977: F, t150980: F, t150983: F, t150985: F, t150988: F, t150992: F) -> (F, F) {
    let t150996 = t89 * t193 * t149920 * t713;
    let t150998 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t141340 + F::cast_from(3.0_f64) * t150953 - F::cast_from(15.0_f64) / F::cast_from(4.0_f64) * t150958 - t150962 / F::cast_from(2.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t150966 + t150971 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t150974 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t150977 + F::cast_from(2.0_f64) * t150980 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t150983 + t141364 - t141368 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t150985 + t150988 / F::cast_from(6.0_f64) + F::cast_from(4.0_f64) * t150992 + F::cast_from(2.0_f64) * t150996;
    (t150996, t150998)
}
