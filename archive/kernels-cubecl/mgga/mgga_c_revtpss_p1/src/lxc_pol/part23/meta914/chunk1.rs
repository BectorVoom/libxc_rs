//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2948/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2948<F: Float>(t4595: F, t63677: F, t4636: F, t64336: F, t15101: F, t19327: F, t15421: F, t19331: F, t19324: F, t52508: F, t19250: F, t19256: F, t52224: F) -> (F, F, F, F, F, F, F) {
    let t78303 = F::cast_from(6.0_f64) * t63677 * t4595;
    let t78305 = F::cast_from(0.48245938496077605201e2_f64) * t64336 * t4636;
    let t78307 = F::cast_from(6.0_f64) * t15101 * t19327;
    let t78309 = F::cast_from(0.48245938496077605201e2_f64) * t15421 * t19331;
    let t78311 = F::cast_from(0.2894756309764656312e3_f64) * t52508 * t19324;
    let t78313 = F::cast_from(0.96491876992155210402e2_f64) * t15421 * t19250;
    let t78315 = F::cast_from(0.1551780387578202009e4_f64) * t52224 * t19256;
    (t78303, t78305, t78307, t78309, t78311, t78313, t78315)
}
