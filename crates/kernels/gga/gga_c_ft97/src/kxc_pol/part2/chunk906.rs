//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 906/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk906<F: Float>(t14012: F, t762: F, t242: F, t1882: F, t3861: F, t3866: F, t1175: F, t2413: F, t724: F, t2405: F, t2594: F, t4005: F, t684: F) -> (F, F, F, F, F, F, F) {
    let t14013 = t762 * t14012;
    let t14014 = t242 * t14013;
    let t14018 = F::new(2.0) / F::new(9.0) * t1882 * t3861;
    let t14020 = F::new(4.0) / F::new(9.0) * t1882 * t3866;
    let t14022 = t724 * t1175 * t2413;
    let t14026 = t2594 * t1175 * t2405;
    let t14030 = t724 * t4005 * t684;
    (t14013, t14014, t14018, t14020, t14022, t14026, t14030)
}
