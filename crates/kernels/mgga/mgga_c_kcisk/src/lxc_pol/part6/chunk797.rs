//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 797/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk797<F: Float>(t642: F, t8831: F, t1806: F, t8537: F, t1850: F, t8491: F, t696: F, t8497: F, t8501: F, t1814: F, t7715: F, t7718: F) -> (F, F, F, F, F, F, F) {
    let t23038 = t8831 * t642;
    let t23068 = t1806 * t8537;
    let t23070 = t1850 * t8491;
    let t23072 = t696 * t8497;
    let t23074 = t1806 * t8501;
    let t23080 = t1814 * t7715;
    let t23096 = t1814 * t7718;
    (t23038, t23068, t23070, t23072, t23074, t23080, t23096)
}
