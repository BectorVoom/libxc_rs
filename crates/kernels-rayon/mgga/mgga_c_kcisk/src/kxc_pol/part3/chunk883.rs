//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 883/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk883(t13009: f64, t420: f64, t1361: f64, t3598: f64, t1175: f64, t3587: f64, t1173: f64, t3616: f64, t12974: f64, t12916: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12948: f64, t12954: f64, t12959: f64, t12985: f64, t12989: f64, t12993: f64, t13010: f64) -> (f64, f64, f64, f64, f64) {
    let t13244 = t13009 * t420;
    let t13247 = t3598 * t1361;
    let t13250 = t1175 * t3587;
    let t13253 = t1173 * t3616;
    let t13263 = 0.12841111111111111111e-1_f64 * t12974;
    let t13274 = 0.14865e-1_f64 * t13010 - 0.2973e-1_f64 * t12916 + 0.1982e-1_f64 * t12993 - t13263 - 0.55033333333333333332e-2_f64 * t12929 + 0.27516666666666666666e-2_f64 * t12933 - 0.82549999999999999999e-2_f64 * t12948 + 0.41274999999999999999e-2_f64 * t12931 - 0.45861111111111111112e-2_f64 * t12922 + 0.1651e-1_f64 * t12954 - 0.82550000000000000001e-2_f64 * t12985 - 0.24765e-1_f64 * t12959 + 0.24765e-1_f64 * t12989 - 0.41275e-2_f64 * t12927;
    (t13244, t13247, t13250, t13253, t13274)
}
