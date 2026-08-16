//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 900/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk900<F: Float>(t17850: F, t9524: F, t2426: F, t5049: F, t5005: F, t694: F, t709: F, t4977: F, t688: F, t200: F, t2379: F, t4960: F) -> (F, F, F, F, F, F, F) {
    let t17877 = t9524 * t17850;
    let t17883 = t2426 * t5049;
    let t17890 = t694 * t5005;
    let t17891 = t17890 * t709;
    let t17894 = t4977 * t688;
    let t17895 = t17894 * t200;
    let t17896 = t2379 * t17895;
    let t17899 = t4960 * t709;
    (t17877, t17883, t17891, t17894, t17895, t17896, t17899)
}
