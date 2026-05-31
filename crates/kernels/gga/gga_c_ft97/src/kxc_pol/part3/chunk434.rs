//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 434/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk434<F: Float>(t3103: F, t370: F, t27: F, t89: F, t1545: F, t1548: F, t1551: F, t2981: F, t2986: F, t2990: F, t2995: F, t3003: F, t3006: F, t3011: F, t3016: F) -> (F, F, F) {
    let t3104 = t370 * t3103;
    let t3106 = t89 * t27 * t3104;
    let t3108 = t1545 + t1548 / F::cast_from(54.0_f64) + t1551 / F::cast_from(18.0_f64) + t2981 / F::cast_from(54.0_f64) - t2986 / F::cast_from(27.0_f64) + t2990 / F::cast_from(18.0_f64) + t2995 / F::cast_from(9.0_f64) - t3003 / F::cast_from(9.0_f64) + t3006 / F::cast_from(18.0_f64) + t3011 / F::cast_from(18.0_f64) + t3016 / F::cast_from(3.0_f64) - t3106 / F::cast_from(6.0_f64);
    (t3104, t3106, t3108)
}
