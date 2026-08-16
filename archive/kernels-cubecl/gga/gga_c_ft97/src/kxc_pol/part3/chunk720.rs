//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 720/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk720<F: Float>(t3882: F, t8392: F, t3888: F, t1882: F, t3979: F, t13746: F, t13753: F, t13780: F, t13794: F, t13809: F, t13811: F, t3861: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13961 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3882;
    let t13963 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8392 * t3888;
    let t13965 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3979;
    let t13983 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13746;
    let t13984 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13753;
    let t13993 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13780;
    let t13998 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13794;
    let t14004 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13809;
    let t14005 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13811;
    let t14018 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3861;
    (t13961, t13963, t13965, t13983, t13984, t13993, t13998, t14004, t14005, t14018)
}
