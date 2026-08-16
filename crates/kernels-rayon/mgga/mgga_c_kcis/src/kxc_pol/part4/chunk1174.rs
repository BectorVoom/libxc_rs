//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1174/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1174(t14868: f64, t5182: f64, t1183: f64, t3436: f64, t1094: f64, t5163: f64, t1172: f64, t10525: f64, t284: f64, t5048: f64, t1175: f64, t5042: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t14869 = t14868 * t5182;
    let t14871 = t1183 * t3436;
    let t14872 = t14871 * t5182;
    let t14874 = t5163 * t1094;
    let t14875 = t14874 * sigma0;
    let t14876 = t14875 * t1172;
    let t14878 = t10525 * t284;
    let t14879 = t14878 * t5048;
    let t14881 = t1175 * t5042;
    (t14869, t14872, t14876, t14879, t14881)
}
