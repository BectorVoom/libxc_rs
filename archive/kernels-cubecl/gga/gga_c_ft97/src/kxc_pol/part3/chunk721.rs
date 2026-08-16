//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 721/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk721<F: Float>(t1882: F, t3866: F, t3844: F, t255: F, t9952: F, t258: F, t9570: F, t9577: F, t1162: F, t2399: F, t89: F, t3871: F, t8392: F) -> (F, F, F, F, F, F, F) {
    let t14020 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1882 * t3866;
    let t14052 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3844;
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14098 = t258 * t9577;
    let t14114 = t89 * t2399 * t1162;
    let t14126 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3871;
    (t14020, t14052, t14080, t14081, t14098, t14114, t14126)
}
