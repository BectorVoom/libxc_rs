//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 345/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk345<F: Float>(t10: F, t144: F, t1542: F, t1546: F, t520: F, t89: F, t375: F, t559: F, t143: F, t1557: F, t378: F, t525: F) -> (F, F, F, F, F, F, F, F) {
    let t1956 = t10 * t1542 * t144;
    let t1957 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1956;
    let t1959 = t89 * t1546 * t520;
    let t1960 = t1959 / F::cast_from(27.0_f64);
    let t1962 = t89 * t375 * t559;
    let t1963 = t1962 / F::cast_from(9.0_f64);
    let t1964 = t143 * t1557;
    let t1969 = t378 * t525;
    (t1956, t1957, t1959, t1960, t1962, t1963, t1964, t1969)
}
