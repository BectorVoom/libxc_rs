//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1174/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1174<F: Float>(t14567: F, t786: F, t2801: F, t10533: F, t10539: F, t10543: F, t10548: F, t10645: F, t10647: F, t10651: F, t10655: F, t14546: F, t14547: F, t14558: F, t14564: F, t2646: F, t2724: F, t2754: F, t4494: F, t4504: F, t4514: F, t4526: F, t820: F) -> F {
    let t14568 = t786 * t14567;
    let t14570 = F::cast_from(0.19514881078765566038e-1_f64) * t14568 * t2801;
    let t14572 = F::cast_from(0.19514881078765566038e-1_f64) * t10533 - F::cast_from(0.23131639038696784278e-2_f64) * t10539 - F::cast_from(0.19514881078765566038e-1_f64) * t10543 - F::cast_from(0.9757440539382783019e-2_f64) * t10548 - F::cast_from(0.39512695097613069591e1_f64) * t14546 * t4494 * t14547 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t4494 * t2724 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t4494 * t2646 - F::cast_from(0.65049603595885220126e-3_f64) * t14558 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4526 * t2754 + F::cast_from(0.13009920719177044025e-1_f64) * t14564 - t10645 - F::cast_from(0.2601984143835408805e-1_f64) * t10647 + t10651 - t14570 - F::cast_from(0.10975748638225852664e-1_f64) * t10655;
    t14572
}
