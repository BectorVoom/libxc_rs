//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2955/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2955<F: Float>(t16558: F, t2989: F, t10224: F, t5828: F, t973: F, t42875: F, t5817: F, t17763: F, t2960: F, t10241: F, t10245: F, t17794: F, t17800: F, t2986: F, t2988: F, t3014: F, t343: F, t4546: F, t48397: F, t48402: F, t48407: F, t48417: F, t48421: F, t5842: F) -> F {
    let t61589 = t2989 * t16558;
    let t61597 = t973 * t10224 * t5828;
    let t61600 = t973 * t42875 * t5817;
    let t61602 = t2960 * t17763;
    let t61614 = -F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t10241 * t17794 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t2988 * t61589 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t17800 * t10245 - F::cast_from(0.6172839506172839506e-4_f64) * t61597 - F::cast_from(0.82304526748971193413e-4_f64) * t61600 + F::cast_from(0.98765432098765432095e-3_f64) * t61602 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t5842 * t3014 * t343 + F::cast_from(0.20576131687242798353e-3_f64) * t48397 - F::cast_from(0.11111111111111111111e-2_f64) * t48402 - F::cast_from(0.55555555555555555554e-3_f64) * t48407 - F::cast_from(0.55555555555555555554e-3_f64) * t48417 + F::cast_from(0.18106995884773662551e-2_f64) * t48421;
    t61614
}
