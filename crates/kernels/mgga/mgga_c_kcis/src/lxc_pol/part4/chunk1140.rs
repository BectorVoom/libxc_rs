//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1140/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1140<F: Float>(t1897: F, t3809: F, t1958: F, t3820: F, t1317: F, t5523: F, t16048: F, t16046: F, t16050: F, t16057: F, t16067: F, t16071: F, t16080: F, t16084: F, t16135: F, t16137: F, t16168: F) -> (F, F, F, F) {
    let t16491 = t1897 * t3809;
    let t16500 = t3820 * t1958;
    let t16503 = t1317 * t5523;
    let t16523 = 0.18344444444444444444e-2 * t16048;
    let t16529 = 0.14865e-1 * t16168 - 0.1982e-1 * t16135 - 0.991e-2 * t16137 - 0.18344444444444444444e-2 * t16046 - 0.55033333333333333333e-2 * t16050 + t16523 - 0.27516666666666666667e-2 * t16071 - 0.45861111111111111112e-2 * t16057 + 0.11006666666666666667e-1 * t16067 + 0.8255e-2 * t16084 - 0.3302e-1 * t16080;
    (t16491, t16500, t16503, t16529)
}
