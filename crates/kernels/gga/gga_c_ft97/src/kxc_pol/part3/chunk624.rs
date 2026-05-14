//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 624/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk624<F: Float>(t26: F, t356: F, t1570: F, t469: F, t11069: F, t11076: F, t11416: F, t100: F, t1587: F, t487: F, t942: F, t1882: F, t3231: F, t3201: F, t8392: F, t3170: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11778 = 2.0 / 9.0 * t11069;
    let t11781 = 4.0 / 27.0 * t11076;
    let t11798 = 4.0 / 9.0 * t11416;
    let t11810 = t1587 * t100;
    let t11811 = t487 * t942;
    let t11821 = 2.0 / 9.0 * t1882 * t3231;
    let t11826 = 2.0 / 27.0 * t8392 * t3201;
    let t11837 = t3170 * t487;
    (t11761, t11762, t11778, t11781, t11798, t11810, t11811, t11821, t11826, t11837)
}
