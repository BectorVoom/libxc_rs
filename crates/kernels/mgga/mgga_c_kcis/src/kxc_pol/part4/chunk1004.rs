//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1004/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1004<F: Float>(t1121: F, t4772: F, t1022: F, t3227: F, t1092: F, t1767: F, t3316: F, t2861: F, t4820: F, t4825: F, t10245: F, t4813: F, t1021: F, t1773: F, t3190: F, t3218: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14092 = t4772 * t1121;
    let t14093 = t1022 * t14092;
    let t14094 = t3227 * t14093;
    let t14095 = t1092 * t14094;
    let t14097 = t1767 * t3316;
    let t14098 = t1022 * t14097;
    let t14099 = t3227 * t14098;
    let t14100 = t1092 * t14099;
    let t14102 = t2861 * t4820;
    let t14103 = 0.66327777777777777776e-2 * t14102;
    let t14104 = t2861 * t4825;
    let t14106 = t10245 * t4813;
    let t14107 = t1021 * t14106;
    let t14108 = t1092 * t14107;
    let t14110 = t1773 * t3190;
    let t14111 = t3218 * t14110;
    (t14092, t14095, t14097, t14100, t14102, t14103, t14104, t14108, t14110, t14111)
}
