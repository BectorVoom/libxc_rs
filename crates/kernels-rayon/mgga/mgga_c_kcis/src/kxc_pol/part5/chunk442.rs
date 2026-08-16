//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 442/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk442(t1040: f64, t1664: f64, t1671: f64, t1036: f64, t1670: f64, t245: f64) -> (f64, f64) {
    let t1724 = 0.1982e-1_f64 * t1671 - t1040 - 0.41275e-2_f64 * t1664;
    let t1727 = t1036 * t1670 / 4.0_f64 + t245 * t1724 / 2.0_f64;
    (t1724, t1727)
}
