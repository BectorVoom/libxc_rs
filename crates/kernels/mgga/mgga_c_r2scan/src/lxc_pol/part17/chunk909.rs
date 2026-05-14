//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 909/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk909<F: Float>(t1020: F, t1129: F, t1131: F, t1133: F, t1135: F, t1137: F, t2410: F, t3526: F, t3530: F, t3534: F, t3538: F, t3542: F, t3749: F, t3753: F, t3757: F, t3761: F, t3765: F, t839: F) -> (F,) {
    let t12338 = -0.9214113627294e1 * t3526 * t1020 - 0.9214113627294e1 * t1129 * t2410 - 0.9214113627294e1 * t3749 * t839 + 0.367387230261e2 * t3530 * t1020 + 0.367387230261e2 * t1131 * t2410 + 0.367387230261e2 * t3753 * t839 - 0.3831420472412e2 * t3534 * t1020 - 0.3831420472412e2 * t1133 * t2410 - 0.3831420472412e2 * t3757 * t839 + 0.1550653405116e2 * t3538 * t1020 + 0.1550653405116e2 * t1135 * t2410 + 0.1550653405116e2 * t3761 * t839 - 0.2177652951264e1 * t3542 * t1020 - 0.2177652951264e1 * t1137 * t2410 - 0.2177652951264e1 * t3765 * t839;
    (t12338,)
}
