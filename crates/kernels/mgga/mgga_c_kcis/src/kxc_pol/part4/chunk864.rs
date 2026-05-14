//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 864/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk864<F: Float>(t2526: F, t9046: F, t9045: F, t2612: F, t8531: F, t2593: F, t62: F, t7617: F, t153: F, t7627: F, t818: F, t8755: F, t9024: F, t9026: F, t9028: F, t9031: F, t9034: F, t9036: F, t9038: F, t9040: F, t9043: F) -> (F, F, F, F, F, F) {
    let t9047 = t9046 * t2526;
    let t9048 = t9045 * t9047;
    let t9050 = t8531 * t2612;
    let t9052 = t2593 * t62;
    let t9053 = t7617 * t2526;
    let t9054 = t9052 * t9053;
    let t9056 = t153 * t7627;
    let t9058 = t8755 * t818;
    let t9060 = 0.3375e1 * t9024 - 0.2428125e1 * t9026 + 0.225e1 * t9028 - 0.485625e0 * t9031 + 0.2428125e1 * t9034 - 0.3375e1 * t9036 - 0.97125e0 * t9038 + 0.485625e0 * t9040 + 0.1125e1 * t9043 - 0.2428125e0 * t9048 - 0.2428125e0 * t9050 + 0.1125e1 * t9054 - 0.45e1 * t9056 + 0.12140625e0 * t9058;
    (t9048, t9050, t9054, t9056, t9058, t9060)
}
