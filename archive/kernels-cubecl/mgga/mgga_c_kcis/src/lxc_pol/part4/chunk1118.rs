//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1118/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1118<F: Float>(t1021: F, t14106: F, t1092: F, t1773: F, t3190: F, t3218: F, t10338: F, t1754: F, t2943: F, t304: F, t2944: F, t4601: F) -> (F, F, F, F, F, F) {
    let t14107 = t1021 * t14106;
    let t14108 = t1092 * t14107;
    let t14110 = t1773 * t3190;
    let t14111 = t3218 * t14110;
    let t14112 = t1021 * t14111;
    let t14113 = t1092 * t14112;
    let t14115 = t10338 * t1754;
    let t14117 = t304 * t2943;
    let t14118 = t4601 * t2944;
    (t14108, t14110, t14113, t14115, t14117, t14118)
}
