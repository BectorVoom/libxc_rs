//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1194/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1194<F: Float>(t22138: F, t5498: F, t1962: F, t5526: F, t1102: F, t11632: F, t11640: F, t16545: F, t16547: F, t16562: F, t16567: F, t16587: F, t1924: F, t22091: F, t22095: F, t22099: F, t22103: F, t22107: F, t22111: F, t22116: F, t22120: F, t22128: F, t22131: F, t22135: F, t344: F, t4587: F, t5623: F) -> (F, F) {
    let t22139 = t5498 * t22138;
    let t22142 = t1962 * t5526;
    let t22143 = t5498 * t22142;
    let t22146 = -t16545 - t16547 + 0.73004774074074074073e-3 * t22091 - 0.1478346675e-2 * t1102 * t22095 + 0.19711289e-2 * t1102 * t22099 - 0.13140859333333333333e-2 * t1102 * t22103 + 0.26281718666666666666e-2 * t4587 * t22107 - 0.19711289e-2 * t1102 * t22111 + 0.59133867e-2 * t1102 * t22116 - 0.19711289e-2 * t11632 * t22120 - 0.14600954814814814815e-3 * t11640 + t16562 + t16567 - 0.87605728888888888887e-3 * t16587 - 8.0 * t1924 * t5623 + 0.1478346675e-2 * t344 * t22128 - 0.19711289e-2 * t22131 + 0.295669335e-2 * t1102 * t22135 - 0.59133867e-2 * t1102 * t22139 + 0.39422578e-2 * t1102 * t22143;
    (t22142, t22146)
}
