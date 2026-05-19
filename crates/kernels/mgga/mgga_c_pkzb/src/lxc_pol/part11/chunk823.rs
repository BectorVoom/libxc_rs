//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 823/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk823<F: Float>(t1536: F, t3401: F, t5149: F, t1020: F, t1535: F, t2536: F, t2718: F, t3396: F, t5091: F, t5130: F, t5139: F, t5141: F, t5148: F, t637: F, t7015: F, t7017: F, t7019: F, t7022: F, t7201: F, t8769: F, t8772: F, t8773: F, t8774: F, t8776: F, t8778: F, t8779: F) -> (F, F) {
    let t8783 = t1536 * t3401;
    let t8789 = F::cast_from(0.11696447245269292414e1_f64) * t5149;
    let t8793 = F::new(6.0) * t1020 * t1535 * t7201 + F::new(3.0) * t1535 * t1536 * t3396 + F::new(2.0) * t2536 * t637 * t8779 + F::new(6.0) * t2718 * t8783 + t5091 - t5130 - t5139 + t5141 - t5148 - t7015 - t7017 + t7019 + t7022 - t8769 - t8772 - t8773 - t8774 + t8776 + t8778 + t8789;
    (t8789, t8793)
}
