//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1172/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1172<F: Float>(t1437: F, t16082: F, t1330: F, t16060: F, t16078: F, t16069: F, t16055: F, t3883: F, t16065: F, t5845: F, t743: F, t5848: F, t1451: F, t1430: F, t542: F, t111: F, t120: F, t12058: F, t12061: F, t12064: F, t4865: F, t4881: F, t5820: F) -> (F,) {
    let t17155 = t1437 * t16082;
    let t17158 = t1330 * t16060;
    let t17161 = t1437 * t16078;
    let t17164 = t1330 * t16069;
    let t17167 = t3883 * t16055;
    let t17170 = t1330 * t16065;
    let t17174 = 0.4705225e-4 * t743 * t5845;
    let t17175 = t743 * t5848;
    let t17177 = t1451 * t16082;
    let t17180 = t1430 * t16060;
    let t17183 = t1451 * t16078;
    let t17186 = t1430 * t16069;
    let t17189 = t542 * t16055;
    let t17192 = t1430 * t16065;
    let t17196 = -0.1585e-2 * t111 * t17155 + 0.317e-2 * t111 * t17158 + 0.634e-2 * t4865 * t17161 - 0.52833333333333333333e-3 * t111 * t17164 - 0.17611111111111111111e-3 * t111 * t17167 + 0.21133333333333333334e-2 * t4865 * t17170 + t17174 + 0.31368166666666666666e-4 * t17175 - 0.10082625e-4 * t120 * t17177 + 0.403305e-4 * t120 * t17180 + 0.403305e-4 * t4881 * t17183 - 0.672175e-5 * t120 * t17186 + 0.22405833333333333333e-5 * t120 * t17189 + 0.26887e-4 * t4881 * t17192 + 0.10359077815592613752e-3 * t5820 - t12058 + t12061 + t12064;
    (t17196,)
}
