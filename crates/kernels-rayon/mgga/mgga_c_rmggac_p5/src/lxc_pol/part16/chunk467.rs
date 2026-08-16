//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 467/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk467(t5828: f64, t814: f64, t1730: f64, t820: f64, t316: f64, t5814: f64, t101: f64, t1580: f64, t1584: f64, t1711: f64, t1715: f64, t1721: f64, t309: f64, t317: f64, t3901: f64, t4861: f64, t4882: f64, t544: f64, t5800: f64, t5804: f64, t5810: f64, t5815: f64, t5825: f64, t87: f64, t98: f64) -> f64 {
    let t5829 = t5828 * t814;
    let t5832 = t820 * t1730;
    let t5833 = t5832 * t316;
    let t5836 = -t5814;
    let t5837 = t101 * t5836;
    let t5840 = -100.0_f64 / 27.0_f64 * t309 * t1711 - 20.0_f64 / 27.0_f64 * t87 * t5800 + 40.0_f64 / 9.0_f64 * t4861 * t5804 - 50.0_f64 / 9.0_f64 * t309 * t1715 + 20.0_f64 / 9.0_f64 * t87 * t5810 + 10.0_f64 / 3.0_f64 * t87 * t5815 + 400.0_f64 / 27.0_f64 * t1721 * t317 - 200.0_f64 / 27.0_f64 * t544 * t1580 + 100.0_f64 / 9.0_f64 * t544 * t1584 - 20.0_f64 / 27.0_f64 * t98 * t5825 - 40.0_f64 / 9.0_f64 * t4882 * t5829 + 20.0_f64 / 9.0_f64 * t98 * t5833 + 10.0_f64 / 3.0_f64 * t98 * t5837 + t3901;
    t5840
}
