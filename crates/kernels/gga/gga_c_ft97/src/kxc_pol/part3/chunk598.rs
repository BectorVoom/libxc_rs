//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 598/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk598<F: Float>(t9071: F, t1984: F, t378: F, t582: F, t597: F, t1554: F, t525: F, t157: F, t1557: F, t604: F, t1570: F, t355: F, t2101: F, t605: F, t151: F, t3051: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9072 = 14.0 / 81.0 * t9071;
    let t9073 = t378 * t1984;
    let t9099 = t582 * t597;
    let t9114 = t1554 * t525;
    let t9115 = t9114 * t157;
    let t9121 = t604 * t1557;
    let t9127 = t604 * t1570;
    let t9132 = t355 * t1984;
    let t9133 = t9132 * t157;
    let t9144 = t2101 * t605;
    let t9166 = 28.0 / 27.0 * t9071;
    let t9178 = 28.0 / 27.0 * t3051 * t151;
    (t9072, t9073, t9099, t9114, t9115, t9121, t9127, t9132, t9133, t9144, t9166, t9178)
}
