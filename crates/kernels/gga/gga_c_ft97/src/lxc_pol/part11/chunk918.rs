//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 918/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk918<F: Float>(t1882: F, t8564: F, t8224: F, t24: F, t32075: F, t1873: F, t8232: F, t8413: F, t1559: F, t1580: F) -> (F, F, F, F, F, F) {
    let t38895 = t1882 * t8564;
    let t38901 = t1882 * t8224;
    let t38921 = t24 * t32075;
    let t38926 = t8232 * t1873;
    let t38928 = t1882 * t8413;
    let t38930 = t1580 * t1559;
    (t38895, t38901, t38921, t38926, t38928, t38930)
}
