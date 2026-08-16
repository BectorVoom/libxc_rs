//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 667/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk667(t1122: f64, t7726: f64, t303: f64, t355: f64, t359: f64, t982: f64, t342: f64, t1134: f64, t356: f64, t2173: f64, t2175: f64, t7687: f64, t7690: f64, t7693: f64, t7696: f64, t7701: f64, t7703: f64, t7706: f64, t7711: f64, t7717: f64, t7721: f64, t7724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7727 = t7726 * t1122;
    let t7728 = t303 * t7727;
    let t7731 = t355 * t982 * t359;
    let t7732 = t342 * t7731;
    let t7733 = t303 * t7732;
    let t7735 = t356 * t1134;
    let t7736 = t303 * t7735;
    let t7738 = -0.69505208333333333333e-3_f64 * t7687 * t2175 + 0.92754700520833333333e-4_f64 * t7690 * t7693 + 0.18534722222222222222e-2_f64 * t7696 * t2175 - t7701 - 0.23168402777777777778e-3_f64 * t7703 * t7706 + 0.69505208333333333333e-3_f64 * t2173 * t7711 + 0.69505208333333333333e-3_f64 * t2173 * t7693 + t7717 + 0.16581944444444444444e-2_f64 * t7721 + 0.24872916666666666666e-2_f64 * t7724 - 0.24872916666666666666e-2_f64 * t7728 - 0.66327777777777777776e-2_f64 * t7733 + 0.16581944444444444444e-2_f64 * t7736;
    (t7727, t7728, t7731, t7732, t7733, t7735, t7736, t7738)
}
