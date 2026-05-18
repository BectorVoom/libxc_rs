//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 696/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk696<F: Float>(t1882: F, t3221: F, t11174: F, t443: F, t444: F, t3283: F, t103: F, t7800: F, t1570: F, t2266: F, t1557: F, t8654: F) -> (F, F, F, F, F, F) {
    let t11999 = F::new(4.0) / F::new(9.0) * t1882 * t3221;
    let t12001 = t443 * t444 * t11174;
    let t12002 = t12001 * t3283;
    let t12020 = t103 * t7800;
    let t12116 = t2266 * t1570;
    let t12122 = t8654 * t1557;
    (t11999, t12001, t12002, t12020, t12116, t12122)
}
