//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1405/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1405<F: Float>(t17738: F, t17962: F, t17988: F, t18008: F, t18034: F, t18054: F, t18237: F, t18263: F, t2118: F, t4479: F, t1636: F, t6256: F) -> (F, F, F) {
    let t18266 = t17738 + t17962 + t17988 + t18008 + t18034 + t18054 + t18237 + t18263;
    let t18268 = t2118 * t4479;
    let t18271 = t6256 * t1636;
    (t18266, t18268, t18271)
}
