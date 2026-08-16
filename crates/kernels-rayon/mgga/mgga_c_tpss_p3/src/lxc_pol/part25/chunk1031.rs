//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1031/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1031(t14256: f64, t2389: f64, t774: f64, t14003: f64, t14034: f64, t14036: f64, t14050: f64, t14053: f64, t14057: f64, t7929: f64, t7932: f64, t7936: f64, t8000: f64, t8019: f64, t8023: f64, t8024: f64, t8029: f64, t8040: f64) -> (f64, f64) {
    let t14258 = t2389 * t774 * t14256;
    let t14261 = t8000 - t14003 - t8019 + t8023 + t8024 + t14034 - t8029 + t14036 - t8040 + t14050 - t14053 + t7929 - t7932 - t7936 - t14057;
    (t14258, t14261)
}
