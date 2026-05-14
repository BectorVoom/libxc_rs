//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1258/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1258<F: Float>(t21172: F, t3626: F, t1261: F, t17412: F, t17444: F, t17447: F, t17453: F, t17474: F, t1808: F, t21153: F, t21157: F, t21161: F, t21166: F, t21170: F, t3625: F, t3647: F, t3718: F, t5331: F, t6673: F) -> (F,) {
    let t21173 = t3626 * t21172;
    let t21176 = 0.23818898954483187207e-3 * t3647 * t6673 + 0.15244095330869239812e-2 * t17412 * t1808 - 0.14291339372689912324e-3 * t1261 * t21153 + t17444 - t17447 - t17453 - 0.14291339372689912324e-3 * t3625 * t21157 - 0.28582678745379824648e-3 * t3625 * t21161 - 0.42874018118069736972e-3 * t3718 * t21166 + t17474 + t21170 / 648.0 + 0.14291339372689912324e-3 * t5331 * t21173;
    (t21176,)
}
