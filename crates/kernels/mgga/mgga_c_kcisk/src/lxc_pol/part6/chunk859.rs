//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 859/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk859<F: Float>(t29494: F, t29496: F, t29499: F, t29501: F, t29505: F, t29507: F, t29514: F, t29517: F, t29520: F, t29524: F, t29526: F, t29529: F, t29531: F, t29535: F, t29537: F, t29542: F, t29545: F, t29548: F, t29551: F, t29554: F, t29556: F, t29558: F, t29562: F, t29565: F, t29567: F, t29569: F, t29573: F, t29576: F, t29578: F, t29581: F) -> (F, F) {
    let t30082 = 0.9375e-1 * t29494 + 0.43166666666666666667e0 * t29496 - 0.50000000000000000001e0 * t29499 - 0.375e0 * t29501 - 0.9375e-1 * t29505 + 0.275e1 * t29507 + 0.25060648148148148148e1 * t29514 + 0.375e0 * t29517 + 0.71944444444444444444e-1 * t29520 + 0.29976851851851851851e-2 * t29524 - 0.625e-1 * t29526 - 0.275e1 * t29529 - 0.60703125e-1 * t29531 + 0.101171875e-1 * t29535 + 0.303515625e-1 * t29537;
    let t30099 = -0.62499999999999999999e-1 * t29542 + 0.10252083333333333334e1 * t29545 + 0.40468749999999999999e-1 * t29548 + 0.5625e0 * t29551 - 0.13489583333333333333e-1 * t29554 + 0.5625e0 * t29556 - 0.28125e0 * t29558 - 0.13489583333333333333e-1 * t29562 - 0.13669444444444444444e1 * t29565 - 0.40468749999999999999e-1 * t29567 + 0.1875e0 * t29569 + 0.625e-1 * t29573 + 0.75e0 * t29576 + 0.303515625e-1 * t29578 + 0.21583333333333333333e0 * t29581;
    (t30082, t30099)
}
