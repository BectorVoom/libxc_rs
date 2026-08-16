//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1056/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1056(t68614: f64, t74491: f64, t74495: f64, t74506: f64, t77031: f64, t77034: f64, t77037: f64, t77043: f64, t77046: f64, t77049: f64, t77052: f64, t77054: f64, t77055: f64, t77056: f64, t77058: f64, t77061: f64, t77062: f64) -> f64 {
    let t80113 = t77031 + t77034 - t77037 - t74491 - 0.40992351065071538967e-4_f64 * t68614 - t77043 + t77046 - t77049 - t77052 - 0.1313947956967602539e-5_f64 * t74495 + t77054 - t77055 + t77056 - t77058 - t77061 + t77062 + t74506;
    t80113
}
