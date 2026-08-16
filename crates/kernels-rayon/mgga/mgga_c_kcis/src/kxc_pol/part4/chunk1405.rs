//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1405/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1405(t17738: f64, t17962: f64, t17988: f64, t18008: f64, t18034: f64, t18054: f64, t18237: f64, t18263: f64, t2118: f64, t4479: f64, t1636: f64, t6256: f64) -> (f64, f64, f64) {
    let t18266 = t17738 + t17962 + t17988 + t18008 + t18034 + t18054 + t18237 + t18263;
    let t18268 = t2118 * t4479;
    let t18271 = t6256 * t1636;
    (t18266, t18268, t18271)
}
