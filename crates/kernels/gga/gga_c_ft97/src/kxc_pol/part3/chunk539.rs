//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 539/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk539<F: Float>(t160: F, t4790: F, t1023: F, t1058: F, t149: F, t165: F, t4650: F, t4720: F, t4725: F, t4806: F, t4810: F, t4837: F, t184: F, t2258: F, t2259: F, t4417: F) -> (F, F, F, F) {
    let t4839 = t4790 * t160;
    let t4844 = -2.0 * t1023 * t1058 - t149 * t4837 - t165 * t4650 - t165 * t4720 + 4.0 * t4725 - 2.0 * t4806 - 4.0 * t4810 + 2.0 * t4839;
    let t4845 = t4844 * t184;
    let t4857 = t2258 * t2259 * t4417;
    (t4839, t4844, t4845, t4857)
}
