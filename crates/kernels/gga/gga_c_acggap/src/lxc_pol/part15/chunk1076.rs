//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1076/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1076<F: Float>(t2176: F, t5368: F, t1620: F, t8331: F, t33796: F, t8313: F, t33799: F, t8310: F, t38086: F, t4210: F, t7942: F, t524: F, t9427: F) -> (F, F, F, F, F, F) {
    let t38361 = t2176 * t5368;
    let t38370 = F::cast_from(0.26341796731742046394e1_f64) * t8331 * t1620;
    let t38377 = F::cast_from(0.17347256376410398924e1_f64) * t33796 * t8313;
    let t38379 = F::cast_from(0.17347256376410398924e1_f64) * t33799 * t8310;
    let t38382 = F::cast_from(0.17347256376410398924e1_f64) * t7942 * t38086 * t4210;
    let t38383 = t9427 * t524;
    (t38361, t38370, t38377, t38379, t38382, t38383)
}
