//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 787/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk787<F: Float>(t11425: F, t556: F, t1404: F, t4035: F, t12048: F, t5796: F, t1401: F, t5808: F, t1445: F, t5789: F, t532: F, t5793: F, t1409: F, t167: F, t5801: F, t5805: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17009 = t556 * t11425;
    let t17019 = t1404 * t4035;
    let t17024 = t12048 * t5796;
    let t17027 = 0.93706135855523581992e-2 * t1401 * t5808;
    let t17045 = 0.93706135855523581992e-2 * t1445 * t5789;
    let t17047 = 0.93706135855523581992e-2 * t532 * t5793;
    let t17057 = t1409 * t167;
    let t17062 = 0.93706135855523581992e-2 * t532 * t5801;
    let t17065 = 0.28111840756657074598e-1 * t1401 * t5805;
    (t17009, t17019, t17024, t17027, t17045, t17047, t17057, t17062, t17065)
}
