//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 964/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk964(t74503: f64, t15523: f64, t2191: f64, t1986: f64, t675: f64, t9566: f64, t68660: f64, t68614: f64, t74495: f64, t74506: f64, t74508: f64, t74511: f64, t74514: f64, t74517: f64, t77043: f64, t77046: f64, t77049: f64, t77052: f64, t77054: f64, t77055: f64) -> f64 {
    let t77056 = 0.1276937996798935182e-4_f64 * t74503;
    let t77057 = t2191 * t15523;
    let t77058 = 0.42564599893297839398e-5_f64 * t77057;
    let t77060 = t675 * t1986 * t9566;
    let t77061 = 0.42564599893297839398e-5_f64 * t77060;
    let t77062 = 0.638468998399467591e-4_f64 * t68660;
    let t77067 = -0.40992351065071538965e-4_f64 * t68614 - t77043 + t77046 - t77049 - t77052 - 0.13139479569676025391e-5_f64 * t74495 + t77054 - t77055 + t77056 - t77058 - t77061 + t77062 + t74506 - 0.3252672799280962148e-5_f64 * t74508 - 0.3252672799280962148e-5_f64 * t74511 - 0.3252672799280962148e-5_f64 * t74514 - 0.3252672799280962148e-5_f64 * t74517;
    t77067
}
