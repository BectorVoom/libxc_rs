//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1145/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1145<F: Float>(t1347: F, t7614: F, t31505: F, t31530: F, t31532: F, t2001: F, t5108: F, t1967: F, t8502: F, t4932: F, t31495: F, t31499: F, t31501: F, t31503: F, t31509: F, t31510: F, t31514: F, t31525: F, t31526: F, t31528: F, t31543: F) -> F {
    let t35709 = t7614 * t1347;
    let t35710 = F::new(0.32012600194825403606e-1) * t35709;
    let t35713 = F::new(0.18007087609589289529e-1) * t31505;
    let t35718 = F::new(0.34299214494455789578e-2) * t31530;
    let t35719 = F::new(0.34299214494455789578e-2) * t31532;
    let t35720 = t2001 * t5108;
    let t35722 = t1967 * t8502;
    let t35723 = F::new(0.25724410870841842184e-2) * t35722;
    let t35724 = t2001 * t4932;
    let t35726 = -t31495 - t31499 - t35710 + F::new(0.32155513588552302729e-2) * t31501 - F::new(0.38586616306262763276e-2) * t31503 - t35713 - t31509 - F::new(7.0) / F::new(144.0) * t31510 - F::new(11.0) / F::new(576.0) * t31514 + t31525 + F::new(0.39624596284901231606e-1) * t31526 + F::new(0.11321313224257494744e-1) * t31528 + t35718 - t35719 + F::new(0.17149607247227894789e-1) * t35720 + t31543 + t35723 + F::new(0.68598428988911579156e-2) * t35724;
    t35726
}
