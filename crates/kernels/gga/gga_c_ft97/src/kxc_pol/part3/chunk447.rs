//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 447/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk447<F: Float>(t3238: F, t492: F, t83: F, t1548: F, t1551: F, t1812: F, t2981: F, t2986: F, t2990: F, t2995: F, t3003: F, t3006: F, t3011: F, t3016: F, t3106: F, t3121: F, t3159: F) -> (F, F, F) {
    let t3239 = t3238 * t492;
    let t3240 = t83 * t3239;
    let t3255 = -t3121 / F::cast_from(4.0_f64) + t3159 / F::cast_from(2.0_f64) + t1812 + t1548 / F::cast_from(9.0_f64) + t1551 / F::cast_from(3.0_f64) + t2981 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2986 + t2990 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2995 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3003 + t3006 / F::cast_from(3.0_f64) + t3011 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t3016 - t3106;
    (t3239, t3240, t3255)
}
