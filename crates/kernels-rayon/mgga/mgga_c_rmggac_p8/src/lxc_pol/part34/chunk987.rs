//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 987/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk987(t26291: f64, t77327: f64, t14444: f64, t1635: f64, t29838: f64, t71219: f64, t74969: f64, t74975: f64, t74979: f64, t77299: f64, t77300: f64, t77301: f64, t77303: f64, t77305: f64, t77309: f64, t77313: f64, t77317: f64, t77321: f64, t77322: f64, t77323: f64) -> (f64, f64) {
    let t77329 = 0.35922725105591425692e0_f64 * t26291 * t77327;
    let t77330 = t14444 * t1635;
    let t77332 = 0.47896966807455234256e0_f64 * t29838 * t77330;
    let t77333 = t77299 - t77300 + t77301 + t77303 - t77305 - t77309 + t77313 - t77317 + t77321 - t77322 - t71219 + t77323 - 0.46594213659335792124e-1_f64 * t74969 + 0.93188427318671584248e-1_f64 * t74975 + 0.15531404553111930708e-1_f64 * t74979 - t77329 + t77332;
    (t77330, t77333)
}
