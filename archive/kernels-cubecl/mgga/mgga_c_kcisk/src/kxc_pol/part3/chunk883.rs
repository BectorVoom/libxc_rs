//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 883/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk883<F: Float>(t13009: F, t420: F, t1361: F, t3598: F, t1175: F, t3587: F, t1173: F, t3616: F, t12974: F, t12916: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12985: F, t12989: F, t12993: F, t13010: F) -> (F, F, F, F, F) {
    let t13244 = t13009 * t420;
    let t13247 = t3598 * t1361;
    let t13250 = t1175 * t3587;
    let t13253 = t1173 * t3616;
    let t13263 = F::cast_from(0.12841111111111111111e-1_f64) * t12974;
    let t13274 = F::cast_from(0.14865e-1_f64) * t13010 - F::cast_from(0.2973e-1_f64) * t12916 + F::cast_from(0.1982e-1_f64) * t12993 - t13263 - F::cast_from(0.55033333333333333332e-2_f64) * t12929 + F::cast_from(0.27516666666666666666e-2_f64) * t12933 - F::cast_from(0.82549999999999999999e-2_f64) * t12948 + F::cast_from(0.41274999999999999999e-2_f64) * t12931 - F::cast_from(0.45861111111111111112e-2_f64) * t12922 + F::cast_from(0.1651e-1_f64) * t12954 - F::cast_from(0.82550000000000000001e-2_f64) * t12985 - F::cast_from(0.24765e-1_f64) * t12959 + F::cast_from(0.24765e-1_f64) * t12989 - F::cast_from(0.41275e-2_f64) * t12927;
    (t13244, t13247, t13250, t13253, t13274)
}
