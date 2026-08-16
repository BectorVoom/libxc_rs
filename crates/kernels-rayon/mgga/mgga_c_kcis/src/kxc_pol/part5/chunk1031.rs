//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1031/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1031(t2880: f64, t421: f64, t4581: f64, t3514: f64, t9959: f64, t4567: f64, t9897: f64, t14496: f64, t1259: f64, t4951: f64, t187: f64, t4731: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15216 = t2880 * t421;
    let t15217 = t15216 * t4581;
    let t15219 = t3514 * t15217 / 432.0_f64;
    let t15220 = t9959 * t421;
    let t15221 = t15220 * t4567;
    let t15223 = t3514 * t15221 / 648.0_f64;
    let t15227 = t9897 * t421;
    let t15231 = t14496 * t421;
    let t15239 = t4951 * t1259;
    let t15296 = t187 * t4731;
    (t15219, t15223, t15227, t15231, t15239, t15296)
}
