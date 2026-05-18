//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1294/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1294<F: Float>(t16179: F, t16182: F, t1045: F, t373: F, t1042: F, t1041: F, t11656: F, t12021: F, t16140: F, t16144: F, t16149: F, t16154: F, t16160: F, t16165: F, t16167: F, t16172: F, t1671: F, t3124: F, t3127: F, t4837: F, t4869: F, t4875: F) -> (F, F) {
    let t16183 = t16179 + t16182;
    let t16185 = t373 * t16183 * t1045;
    let t16186 = t1042 * t16185;
    let t16189 = -F::new(0.28582678745379824648e-3) * t3127 * t16140 + F::new(0.28582678745379824648e-3) * t3127 * t16144 + F::new(0.28582678745379824648e-3) * t4837 * t16149 + F::new(0.85748036236139473944e-3) * t4837 * t16154 + t16160 + F::new(0.15244095330869239812e-2) * t11656 * t4875 + t16165 - F::new(0.14291339372689912324e-3) * t3127 * t16167 - F::new(0.23818898954483187207e-3) * t3127 * t16172 + F::new(0.21437009059034868486e-3) * t12021 * t1671 + F::new(0.42874018118069736972e-3) * t3124 * t4869 + F::new(0.21437009059034868486e-3) * t1041 * t16186;
    (t16183, t16189)
}
