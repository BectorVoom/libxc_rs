//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 954/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk954<F: Float>(t3255: F, t5495: F, t5500: F, t3820: F, t509: F, t1409: F, t1897: F, t1098: F, t5483: F, t1992: F, t3251: F, t1958: F, t1317: F, t5523: F, t16048: F, t5432: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16408 = 0.19711289e-2 * t3255 * t5495;
    let t16410 = 0.26281718666666666666e-2 * t3255 * t5500;
    let t16411 = t509 * t3820;
    let t16416 = t1409 * t1897;
    let t16436 = 0.19711289e-2 * t1098 * t5483;
    let t16441 = t3251 * t1992;
    let t16500 = t3820 * t1958;
    let t16503 = t1317 * t5523;
    let t16523 = 0.18344444444444444444e-2 * t16048;
    let t16543 = 0.13140859333333333334e-2 * t3255 * t5432;
    (t16408, t16410, t16411, t16416, t16436, t16441, t16500, t16503, t16523, t16543)
}
