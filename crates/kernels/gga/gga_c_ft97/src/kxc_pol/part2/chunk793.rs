//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 793/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk793<F: Float>(t11088: F, t120: F, t72: F, t422: F, t929: F, t383: F, t8966: F, t11145: F, t71: F, t1595: F, t528: F, t1005: F) -> (F, F, F, F, F) {
    let t12471 = t11088 * t120;
    let t12472 = t72 * t12471;
    let t12477 = t422 * t929;
    let t12478 = t12477 * t383;
    let t12479 = t12478 * t8966;
    let t12483 = t72 * t11145 * t120;
    let t12486 = t71 * t929;
    let t12488 = t1595 * t528 * t120;
    let t12489 = t12486 * t12488;
    let t12492 = t1005 * t1595;
    (t12472, t12479, t12483, t12489, t12492)
}
