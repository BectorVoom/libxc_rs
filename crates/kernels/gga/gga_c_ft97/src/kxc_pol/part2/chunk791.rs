//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 791/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk791<F: Float>(t1882: F, t3861: F, t3866: F, t1175: F, t2413: F, t724: F, t2405: F, t2594: F, t4005: F, t684: F, t2526: F, t3977: F, t242: F, t10153: F, t1168: F, t13952: F, t13955: F, t13959: F, t13961: F, t13963: F, t13965: F, t13967: F, t14014: F, t1901: F, t446: F) -> (F, F, F) {
    let t14018 = 2.0 / 9.0 * t1882 * t3861;
    let t14020 = 4.0 / 9.0 * t1882 * t3866;
    let t14022 = t724 * t1175 * t2413;
    let t14026 = t2594 * t1175 * t2405;
    let t14030 = t724 * t4005 * t684;
    let t14033 = t3977 * t2526;
    let t14034 = t242 * t14033;
    let t14037 = t10153 * t1168;
    let t14038 = t242 * t14037;
    let t14041 = 2.0 / 9.0 * t1901 * t13952 + 2.0 / 9.0 * t1901 * t13955 - t13959 - t13961 - t13963 + t13965 - 2.0 / 3.0 * t446 * t13967 - t446 * t14014 / 3.0 - t14018 - t14020 - t446 * t14022 / 9.0 - 2.0 / 27.0 * t446 * t14026 - 2.0 / 9.0 * t446 * t14030 - t446 * t14034 / 3.0 - t446 * t14038 / 3.0;
    (t14033, t14037, t14041)
}
