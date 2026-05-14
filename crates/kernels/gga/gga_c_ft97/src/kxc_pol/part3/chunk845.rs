//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 845/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk845<F: Float>(t291: F, t39: F, t4092: F, t2035: F, t5266: F, t811: F, t817: F, t1200: F, t820: F, t283: F, t1197: F, t3780: F, t4125: F, t1701: F, t17975: F, t800: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19100 = t291 * t39;
    let t19101 = t4092 * t19100;
    let t19103 = t2035 * t5266 * t811;
    let t19106 = t817 * t39;
    let t19107 = t1200 * t19106;
    let t19108 = t5266 * t820;
    let t19109 = t2035 * t19108;
    let t19116 = t811 * t283;
    let t19117 = t19116 * t1197;
    let t19120 = t3780 * t4125;
    let t19121 = t1701 * t19120;
    let t19125 = t1701 * t17975 * t811;
    let t19128 = t17975 * t820;
    let t19129 = t1701 * t19128;
    let t19132 = t800 * t19100;
    (t19101, t19103, t19106, t19107, t19109, t19117, t19121, t19125, t19129, t19132)
}
