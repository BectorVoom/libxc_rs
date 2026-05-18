//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 969/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk969<F: Float>(t12717: F, t12718: F, t16195: F, t16198: F, t16201: F, t16204: F, t16207: F, t16210: F, t16213: F, t16215: F, t16219: F, t11475: F, t16046: F, t16052: F, t16057: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16127: F, t16129: F, t16132: F, t16135: F, t16137: F, t16142: F, t16146: F, t16163: F, t16165: F, t16168: F, t17847: F, t17861: F) -> F {
    let t17883 = F::new(0.3529725e1) * t16195 + F::new(0.20839e0) * t16198 - F::new(0.62517e0) * t16201 - F::new(0.34731666666666666667e-1) * t16204 - F::new(0.46308888888888888889e-1) * t16207 + F::new(0.13892666666666666667e0) * t16210 + F::new(0.20839e0) * t16213 + F::new(0.6311625e0) * t16215 - t12717 - t12718 + F::new(0.20839e0) * t16219;
    let t17885 = -F::new(0.90302333333333333334e0) * t16127 - F::new(0.11577222222222222222e0) * t16129 - F::new(0.157790625e0) * t16132 - F::new(0.3529725e1) * t16135 - F::new(0.17648625e1) * t16137 - F::new(0.37874833333333333334e1) * t16052 - F::new(0.22954444444444444444e0) * t16046 - F::new(0.83356e0) * t16142 - t17847 + F::new(0.46308888888888888889e-1) * t16146 + t17861 + F::new(0.6311625e0) * t16163 + F::new(0.31558125e0) * t16165 + F::new(0.264729375e1) * t16168 - F::new(0.57386111111111111112e0) * t16057 + F::new(0.13772666666666666667e1) * t16067 - F::new(0.34431666666666666667e0) * t16071 - F::new(0.309885e1) * t16075 - F::new(0.41318e1) * t16080 + F::new(0.103295e1) * t16084 - F::new(0.13892666666666666667e0) * t11475 + t17883;
    t17885
}
