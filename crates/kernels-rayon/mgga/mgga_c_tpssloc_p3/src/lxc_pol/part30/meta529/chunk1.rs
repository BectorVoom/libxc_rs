//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1871/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1871(t26277: f64, t6926: f64, t22784: f64, t22795: f64, t26255: f64, t26258: f64, t26260: f64, t26262: f64, t26266: f64, t26268: f64, t26272: f64, t26274: f64) -> f64 {
    let t26278 = t6926 * t26277;
    let t26280 = 7.0_f64 / 576.0_f64 * t26255 - t26258 / 384.0_f64 - t26260 / 384.0_f64 - t26262 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t22784 + 0.20186378047070195427e-3_f64 * t22795 + 7.0_f64 / 144.0_f64 * t26266 + 0.84782787797694820792e-2_f64 * t26268 + 0.20186378047070195427e-3_f64 * t26272 - t26274 / 48.0_f64 - 0.12111826828242117256e-2_f64 * t26278;
    t26280
}
