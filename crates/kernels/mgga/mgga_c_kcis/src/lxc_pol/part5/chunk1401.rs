//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1401/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1401<F: Float>(t1889: F, t6183: F, t4440: F, t2104: F, t5477: F, t18128: F, t1610: F, t6912: F, t21106: F, t6159: F, t6937: F, t12617: F) -> (F, F, F, F, F) {
    let t23114 = t1889 * t6183;
    let t23115 = t4440 * t23114;
    let t23118 = t5477 * t2104;
    let t23119 = t18128 * t23118;
    let t23122 = t6912 * t1610;
    let t23123 = t4440 * t23122;
    let t23126 = t6159 * t21106;
    let t23129 = t6937 * t1610;
    let t23130 = t12617 * t23129;
    (t23115, t23119, t23123, t23126, t23130)
}
