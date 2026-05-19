//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1295/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1295<F: Float>(t27555: F, t6140: F, t18210: F, t28771: F, t7978: F, t209: F, t2095: F, t7967: F, t16694: F, t27560: F, t27569: F, t27583: F, t28714: F, t28765: F, t6159: F, t7971: F, t94319: F, t94321: F, t94974: F, t94977: F, t94979: F, t94981: F) -> (F, F, F) {
    let t99233 = t27555 * t6140;
    let t99236 = t18210 * t28771;
    let t99238 = F::cast_from(0.23168402777777777778e-3_f64) * t7978 * t99236;
    let t99247 = t2095 * t209;
    let t99248 = t7967 * t99247;
    let t99255 = -F::cast_from(0.24734586805555555556e-3_f64) * t99233 * t7971 + t99238 + F::cast_from(0.77382407407407407407e-3_f64) * t94319 + F::cast_from(0.12897067901234567901e-2_f64) * t94321 - F::cast_from(0.15445601851851851852e-3_f64) * t94974 + F::cast_from(0.34752604166666666667e-3_f64) * t28714 * t27560 - F::cast_from(0.15445601851851851852e-3_f64) * t94977 - F::cast_from(0.20612155671296296296e-4_f64) * t94979 + F::cast_from(0.23168402777777777778e-3_f64) * t94981 - F::cast_from(0.82448622685185185186e-4_f64) * t99248 * t27569 - F::cast_from(0.69505208333333333334e-3_f64) * t27583 * t6159 * t28765 * t16694;
    (t99236, t99247, t99255)
}
