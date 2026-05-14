//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 961/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk961<F: Float>(t1444: F, t16968: F, t25: F, t5733: F, t493: F, t11425: F, t556: F, t1404: F, t4035: F, t12048: F, t5796: F, t1401: F, t5808: F, t1445: F, t5789: F, t532: F, t5793: F) -> (F, F, F, F, F, F, F, F) {
    let t16969 = t16968 * t1444;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / 144.0;
    let t17009 = t556 * t11425;
    let t17019 = t1404 * t4035;
    let t17024 = t12048 * t5796;
    let t17027 = 0.93706135855523581992e-2 * t1401 * t5808;
    let t17045 = 0.93706135855523581992e-2 * t1445 * t5789;
    let t17047 = 0.93706135855523581992e-2 * t532 * t5793;
    (t16969, t16981, t17009, t17019, t17024, t17027, t17045, t17047)
}
