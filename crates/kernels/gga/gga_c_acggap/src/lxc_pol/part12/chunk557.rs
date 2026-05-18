//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 557/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk557<F: Float>(t460: F, t848: F, t183: F, t3645: F, t1265: F, t857: F, t1210: F, t315: F, t323: F, t188: F, t119: F, t441: F, t862: F) -> (F, F, F, F, F, F, F) {
    let t3843 = t848 * t460;
    let t3846 = F::new(0.65854491829355115987e0) * t3645 * t183;
    let t3856 = t857 * t1265;
    let t3858 = t315 * t1210;
    let t3859 = t3858 * t323;
    let t3862 = F::new(0.65854491829355115987e0) * t3645 * t188;
    let t3865 = t119 * t1210;
    let t3868 = t862 * t441;
    (t3843, t3846, t3856, t3859, t3862, t3865, t3868)
}
