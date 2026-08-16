//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 285/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk285(t920: f64, t924: f64, t935: f64, t1036: f64, t245: f64, t934: f64) -> (f64, f64, f64) {
    let t1040 = 0.41275e-2_f64 * t920;
    let t1042 = 0.1982e-1_f64 * t935 - t1040 - 0.41275e-2_f64 * t924;
    let t1045 = t1036 * t934 / 4.0_f64 + t245 * t1042 / 2.0_f64;
    (t1040, t1042, t1045)
}
