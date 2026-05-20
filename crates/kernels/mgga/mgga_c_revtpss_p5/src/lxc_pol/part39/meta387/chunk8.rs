//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1395/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1395<F: Float>(t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F) -> (F, F) {
    let t17066 = F::cast_from(0.27785333333333333334e0_f64) * t16892;
    let t17075 = F::cast_from(0.22954444444444444444e0_f64) * t16708;
    let t17083 = F::cast_from(0.46308888888888888889e-1_f64) * t16908 + F::new(0.6311625e0) * t16927 - F::cast_from(0.68863333333333333333e0_f64) * t16710 + t17075 + F::cast_from(0.46308888888888888889e-1_f64) * t16931 + F::new(0.3529725e1) * t16933 - F::new(0.20659e1) * t16722 + F::new(0.20659e1) * t16740 + F::new(0.103295e1) * t16744 + F::new(0.309885e1) * t16735 + F::cast_from(0.57386111111111111112e0_f64) * t16717;
    (t17066, t17083)
}
