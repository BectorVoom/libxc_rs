//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 773/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk773<F: Float>(t363: F, t4431: F, t3187: F, t1909: F, t3194: F, t3193: F, t11902: F, t3205: F, t11430: F, t11436: F, t11448: F, t15978: F, t15980: F, t15983: F, t15987: F, t15991: F, t15996: F, t16000: F, t16003: F, t16008: F, t1901: F, t3281: F, t446: F) -> (F, F) {
    let t16011 = t4431 * t363;
    let t16012 = t3187 * t16011;
    let t16013 = t1909 * t16012;
    let t16016 = t3194 * t16011;
    let t16017 = t3193 * t16016;
    let t16020 = t11902 * t3205;
    let t16023 = t15978 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15980 + t11430 - t11436 - t11448 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t15983 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3281 * t15987 - t446 * t15991 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t15996 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16000 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t16003 + t1901 * t16008 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16013 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t16017 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16020;
    (t16011, t16023)
}
