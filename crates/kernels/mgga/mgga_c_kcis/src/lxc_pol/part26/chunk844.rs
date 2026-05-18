//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 844/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk844<F: Float>(t1444: F, t16968: F, t25: F, t5733: F, t493: F, t11425: F, t556: F, t1404: F, t4035: F, t12048: F, t5796: F, t1401: F, t5808: F) -> (F, F, F, F, F, F) {
    let t16969 = t16968 * t1444;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / F::new(144.0);
    let t17009 = t556 * t11425;
    let t17019 = t1404 * t4035;
    let t17024 = t12048 * t5796;
    let t17027 = F::new(0.93706135855523581992e-2) * t1401 * t5808;
    (t16969, t16981, t17009, t17019, t17024, t17027)
}
