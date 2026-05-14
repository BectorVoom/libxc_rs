//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 852/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk852<F: Float>(t2217: F, t862: F, t865: F, t2131: F, t2147: F, t463: F, t8103: F, t2176: F, t3889: F, t8111: F, t872: F, t323: F, t851: F, t633: F, t848: F, t464: F) -> (F, F, F, F, F, F, F) {
    let t33047 = t862 * t2217 * t865;
    let t33053 = t2131 * t2147 * t8103 * t463;
    let t33063 = t2176 * t3889;
    let t33065 = t8111 * t872;
    let t33080 = t851 * t2217 * t323;
    let t33092 = t848 * t633;
    let t33093 = t33092 * t464;
    (t33047, t33053, t33063, t33065, t33080, t33092, t33093)
}
