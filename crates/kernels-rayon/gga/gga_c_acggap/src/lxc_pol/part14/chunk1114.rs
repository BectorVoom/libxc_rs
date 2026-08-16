//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1114/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1114(t1784: f64, t2020: f64, t30664: f64, t30670: f64, t30672: f64, t30673: f64, t30715: f64, t30717: f64, t34660: f64, t34675: f64, t34703: f64, t34704: f64, t34711: f64, t34713: f64, t34718: f64, t37211: f64, t37220: f64, t37221: f64, t37225: f64) -> f64 {
    let t39427 = t2020 * t1784;
    let t39432 = -t30664 - t30670 + t30672 - 0.17149607247227894789e-2_f64 * t30673 + t34660 + 0.41930789719472202757e-3_f64 * t34675 - 7.0_f64 / 144.0_f64 * t39427 - t37211 - t34703 - 0.77173232612525526552e-2_f64 * t34704 + t34711 + t34713 - 0.51448821741683684367e-2_f64 * t34718 + t37220 + t37221 + t37225 - t30715 - 35.0_f64 / 216.0_f64 * t30717;
    t39432
}
