//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 860/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk860<F: Float>(t5309: F, t875: F, t10697: F, t296: F, t1248: F, t4299: F, t2843: F, t5424: F, t824: F, t840: F, t2862: F, t5225: F, t882: F, t1882: F, t5403: F, t5399: F) -> (F, F, F, F, F, F, F, F) {
    let t19430 = t5309 * t875;
    let t19431 = t10697 * t19430;
    let t19432 = t296 * t19431;
    let t19435 = t1248 * t4299;
    let t19436 = t2843 * t19435;
    let t19437 = t296 * t19436;
    let t19442 = t840 * t5424 * t824;
    let t19446 = t2862 * t882 * t5225;
    let t19449 = t1882 * t5403;
    let t19451 = t1882 * t5399;
    (t19431, t19432, t19436, t19437, t19442, t19446, t19449, t19451)
}
