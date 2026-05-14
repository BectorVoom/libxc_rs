//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 724/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk724<F: Float>(t12356: F, t12362: F, t12365: F, t12353: F, t12359: F, t12564: F, t12568: F, t8799: F, t8802: F, t9059: F, t9383: F, t12571: F, t1039: F, t2087: F, t91: F, t9252: F) -> (F, F, F) {
    let t12911 = 4.0 / 9.0 * t12356;
    let t12913 = 4.0 / 81.0 * t12362;
    let t12914 = 2.0 / 9.0 * t12365;
    let t12917 = t8799 / 27.0 + 2.0 / 81.0 * t8802 - 2.0 / 27.0 * t9059 + 4.0 / 3.0 * t12353 - t12911 + 22.0 / 27.0 * t12359 - t12913 - t9383 + t12914 - t12564 / 3.0 - 2.0 / 9.0 * t12568;
    let t12918 = 4.0 / 27.0 * t12571;
    let t12921 = t91 * t9252 * t1039 * t2087;
    (t12917, t12918, t12921)
}
