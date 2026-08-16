//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1992/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1992(t102567: f64, t108615: f64, t108617: f64, t108619: f64, t108623: f64, t108625: f64, t108627: f64, t108629: f64, t94554: f64, t96358: f64, t96359: f64, t98283: f64, t98285: f64) -> f64 {
    let t109839 = -0.30488190661738479625e-3_f64 * t94554 + t108615 / 8.0_f64 - t108617 / 2.0_f64 + t108619 / 4.0_f64 + t102567 - t98283 - t96358 - t96359 - 0.14457274399185490173e-3_f64 * t98285 + 0.28582678745379824648e-4_f64 * t108623 + 0.10164000561857065645e-2_f64 * t108625 - 0.80031500487063509015e-1_f64 * t108627 + 0.16006300097412701803e-1_f64 * t108629;
    t109839
}
