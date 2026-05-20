//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2092/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2092<F: Float>(t2435: F, t27965: F, t14090: F, t26054: F, t14268: F, t2022: F, t7295: F, t7296: F, t7921: F, t94608: F, t94610: F, t94613: F, t94616: F, t97792: F, t97795: F, t97798: F, t97800: F, t97804: F, t97808: F, t97810: F, t97815: F) -> F {
    let t97823 = t2435 * t27965;
    let t97825 = t26054 * t14090;
    let t97827 = F::cast_from(0.73171657588172351096e-2_f64) * t97792 + F::cast_from(0.65049603595885220126e-3_f64) * t97795 - t97798 - F::cast_from(0.22849835011101738147e-2_f64) * t97800 - t94608 - t97804 + F::cast_from(0.25702851531048074406e-1_f64) * t94613 + t97808 + F::cast_from(0.11565819519348392139e-2_f64) * t97810 + F::cast_from(0.23131639038696784278e-2_f64) * t94616 + F::cast_from(0.45699670022203476294e-2_f64) * t97815 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t2022 * t14268 + F::cast_from(0.8673628188205199462e0_f64) * t94610 * t7921 - F::cast_from(0.73171657588172351096e-2_f64) * t97823 + F::cast_from(0.13009920719177044025e-1_f64) * t97825;
    t97827
}
