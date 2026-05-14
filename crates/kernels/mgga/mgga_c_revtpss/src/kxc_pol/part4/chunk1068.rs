//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1068/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1068<F: Float>(t10867: F, t225: F, t213: F, t10871: F, t2722: F, t2777: F, t4518: F, t2439: F, t2470: F, t4499: F, t2798: F, t1568: F, t2783: F, t786: F, t2801: F, t10533: F, t10539: F, t10543: F, t10548: F, t10645: F, t10647: F, t10651: F, t10655: F, t2646: F, t2724: F, t2754: F, t4494: F, t4504: F, t4514: F, t4526: F, t820: F) -> (F, F) {
    let t14545 = t225 * t10867;
    let t14546 = t213 * t14545;
    let t14547 = t10871 * t2722;
    let t14557 = t2777 * t4518;
    let t14558 = t2439 * t14557;
    let t14563 = t4499 * t2470;
    let t14564 = t2798 * t14563;
    let t14567 = t2783 * t1568;
    let t14568 = t786 * t14567;
    let t14570 = 0.19514881078765566038e-1 * t14568 * t2801;
    let t14572 = 0.19514881078765566038e-1 * t10533 - 0.23131639038696784278e-2 * t10539 - 0.19514881078765566038e-1 * t10543 - 0.9757440539382783019e-2 * t10548 - 0.39512695097613069591e1 * t14546 * t4494 * t14547 + 0.39512695097613069591e1 * t4504 * t4494 * t2724 - 0.65854491829355115987e0 * t4514 * t4494 * t2646 - 0.65049603595885220126e-3 * t14558 - 0.65854491829355115987e0 * t820 * t4526 * t2754 + 0.13009920719177044025e-1 * t14564 - t10645 - 0.2601984143835408805e-1 * t10647 + t10651 - t14570 - 0.10975748638225852664e-1 * t10655;
    (t14547, t14572)
}
