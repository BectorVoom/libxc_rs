//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1288/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1288<F: Float>(t16048: F, t16046: F, t16050: F, t16057: F, t16067: F, t16071: F, t16080: F, t16084: F, t16135: F, t16137: F, t16168: F, t11409: F, t11411: F, t11413: F, t11415: F, t11746: F, t16052: F, t16062: F, t16075: F, t16088: F, t16195: F) -> F {
    let t16523 = F::cast_from(0.18344444444444444444e-2_f64) * t16048;
    let t16529 = F::new(0.14865e-1) * t16168 - F::new(0.1982e-1) * t16135 - F::new(0.991e-2) * t16137 - F::cast_from(0.18344444444444444444e-2_f64) * t16046 - F::cast_from(0.55033333333333333333e-2_f64) * t16050 + t16523 - F::cast_from(0.27516666666666666667e-2_f64) * t16071 - F::cast_from(0.45861111111111111112e-2_f64) * t16057 + F::cast_from(0.11006666666666666667e-1_f64) * t16067 + F::new(0.8255e-2) * t16084 - F::new(0.3302e-1) * t16080;
    let t16530 = F::new(0.1651e-1) * t16062 - F::cast_from(0.30268333333333333334e-1_f64) * t16052 + F::new(0.8255e-2) * t16088 + F::new(0.1982e-1) * t16195 - F::cast_from(0.36688888888888888888e-2_f64) * t11409 + F::cast_from(0.13758333333333333333e-2_f64) * t11415 + F::cast_from(0.9172222222222222222e-3_f64) * t11411 - F::new(0.24765e-1) * t16075 - F::cast_from(0.27516666666666666666e-2_f64) * t11413 - t11746 + t16529;
    t16530
}
