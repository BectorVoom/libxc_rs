//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1140/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1140<F: Float>(t1636: F, t2740: F, t89: F, t10498: F, t9582: F, t10409: F, t446: F, t43511: F, t43516: F, t43519: F, t43522: F, t43528: F, t43531: F, t43534: F, t43538: F, t43541: F, t43551: F, t43926: F, t43930: F, t43933: F) -> (F, F, F, F) {
    let t43936 = t89 * t1636 * t2740;
    let t43938 = t9582 * t10498;
    let t43940 = t446 * t10409 * t43938;
    let t43942 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43511 + F::cast_from(2.0_f64) * t43516 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t43519 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43522 + F::cast_from(8.0_f64) * t43528 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43531 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t43534 + t43538 - t43541 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t43551 + t43926 / F::cast_from(6.0_f64) - t43930 / F::cast_from(3.0_f64) + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t43933 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43936 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43940;
    (t43936, t43938, t43940, t43942)
}
