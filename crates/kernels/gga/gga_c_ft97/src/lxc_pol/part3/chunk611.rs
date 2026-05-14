//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 611/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk611<F: Float>(t2344: F, t798: F, t309: F, t2347: F, t870: F, t2680: F, t665: F, t2360: F, t2399: F, t865: F, t89: F, t10400: F, t295: F, t9567: F, t8282: F, t849: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10478 = t2344 * t798;
    let t10479 = t10478 * t309;
    let t10485 = t870 * t2347;
    let t10491 = t665 * t2680;
    let t10492 = t10491 * t309;
    let t10503 = t870 * t2360;
    let t10514 = t89 * t2399 * t865;
    let t10553 = 4.0 / 9.0 * t10400;
    let t10580 = t9567 * t295;
    let t10586 = t8282 * t849;
    (t10478, t10479, t10485, t10491, t10492, t10503, t10514, t10553, t10580, t10586)
}
