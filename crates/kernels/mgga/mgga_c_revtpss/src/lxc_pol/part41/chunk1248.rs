//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1248/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1248<F: Float>(t17202: F, t19661: F, t1042: F, t1261: F, t12855: F, t12967: F, t17362: F, t17569: F, t17709: F, t17747: F, t20959: F, t20963: F, t20966: F, t20974: F, t20978: F, t20982: F, t3647: F, t5299: F, t5391: F, t5397: F, t6611: F, t6679: F) -> (F,) {
    let t20985 = t17202 * t19661;
    let t20986 = t1042 * t20985;
    let t20993 = 0.12862205435420921092e-2 * t17709 * t20959 - 0.12862205435420921092e-2 * t17747 * t20963 + 11.0 / 324.0 * t20966 + 0.15244095330869239812e-2 * t5391 * t5397 - 0.14291339372689912324e-3 * t3647 * t6679 - 0.95275595817932748827e-4 * t20974 - 0.95275595817932748827e-4 * t17362 - 0.42874018118069736972e-3 * t12855 * t20978 - 0.57165357490759649296e-3 * t1261 * t20982 - 0.85748036236139473944e-3 * t1261 * t20986 + 0.28582678745379824648e-3 * t17569 * t5299 + 0.42874018118069736972e-3 * t12967 * t6611;
    (t20993,)
}
