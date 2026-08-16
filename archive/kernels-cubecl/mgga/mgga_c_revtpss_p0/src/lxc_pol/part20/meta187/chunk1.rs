//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 936/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk936<F: Float>(t225: F, t9801: F, t4062: F, t125: F, t4056: F, t3936: F, t3938: F, t3889: F, t543: F, t3937: F, t1386: F, t2482: F, t814: F) -> (F, F, F, F, F, F, F) {
    let t9802 = t9801 * t225;
    let t9804 = F::cast_from(0.45738002528356795401e-4_f64) * t9802 * t4062;
    let t9805 = t125 * t4056;
    let t9807 = t3936 * t9805 * t3938;
    let t9810 = t543 * t3889;
    let t9812 = t3936 * t3937 * t9810;
    let t9816 = t2482 * t1386 * t814;
    (t9802, t9804, t9805, t9807, t9810, t9812, t9816)
}
