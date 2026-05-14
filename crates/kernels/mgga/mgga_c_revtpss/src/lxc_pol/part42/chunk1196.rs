//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1196/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1196<F: Float>(t20069: F, t3117: F, t1063: F, t11774: F, t15926: F, t20040: F, t20046: F, t20051: F, t20055: F, t20066: F, t3106: F, t3188: F, t4892: F, t4899: F, t4912: F, t6323: F, t6327: F, t6331: F) -> (F,) {
    let t20070 = t3117 * t20069;
    let t20073 = -0.28582678745379824648e-3 * t11774 * t20040 + 0.14291339372689912324e-3 * t3188 * t6323 + 0.14291339372689912324e-3 * t1063 * t20046 + 0.15879265969655458138e-3 * t20051 + 0.95275595817932748827e-4 * t20055 - 0.1270341277572436651e-2 * t3106 * t6327 - 0.76220476654346199061e-3 * t3106 * t6323 - 0.28582678745379824648e-3 * t3188 * t6331 - 0.42874018118069736972e-3 * t15926 * t4912 + 0.42874018118069736972e-3 * t4892 * t20066 - 0.21437009059034868486e-3 * t4899 * t20070;
    (t20073,)
}
