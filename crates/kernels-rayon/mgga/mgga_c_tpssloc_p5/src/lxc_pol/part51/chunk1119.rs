//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1119/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1119(t225: f64, t7910: f64, t26231: f64, t26251: f64, t26255: f64, t26266: f64, t22785: f64, t22795: f64, t26258: f64, t26260: f64, t26262: f64, t26268: f64, t26272: f64, t26274: f64, t26278: f64) -> (f64, f64, f64, f64) {
    let t27009 = t7910 * t225;
    let t27012 = 7.0_f64 / 1152.0_f64 * t26231;
    let t27019 = 7.0_f64 / 1152.0_f64 * t26251;
    let t27022 = 7.0_f64 / 288.0_f64 * t26255;
    let t27027 = 7.0_f64 / 72.0_f64 * t26266;
    let t27032 = t27022 - t26258 / 192.0_f64 - t26260 / 192.0_f64 - t26262 / 192.0_f64 + t22785 + 0.40372756094140390853e-3_f64 * t22795 + t27027 + 0.16956557559538964158e-1_f64 * t26268 + 0.40372756094140390853e-3_f64 * t26272 - t26274 / 24.0_f64 - 0.24223653656484234512e-2_f64 * t26278;
    (t27009, t27012, t27019, t27032)
}
