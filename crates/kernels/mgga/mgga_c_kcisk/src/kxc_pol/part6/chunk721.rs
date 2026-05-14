//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 721/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk721<F: Float>(t1644: F, t8544: F, t682: F, t8522: F, t8504: F, t1417: F, t8928: F, t719: F, t8831: F, t642: F, t1806: F, t8537: F, t1850: F, t8491: F, t696: F, t8497: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22891 = t8544 * t1644;
    let t22927 = t682 * t8522;
    let t22937 = t682 * t8504;
    let t22942 = t1417 * t8928;
    let t23033 = t8831 * t719;
    let t23038 = t8831 * t642;
    let t23068 = t1806 * t8537;
    let t23070 = t1850 * t8491;
    let t23072 = t696 * t8497;
    (t22891, t22927, t22937, t22942, t23033, t23038, t23068, t23070, t23072)
}
