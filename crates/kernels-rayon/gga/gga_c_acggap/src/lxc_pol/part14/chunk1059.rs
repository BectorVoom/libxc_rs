//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1059/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1059(t33831: f64, t33840: f64, t33842: f64, t33844: f64, t33852: f64, t33853: f64, t36823: f64, t38701: f64, t38704: f64, t38706: f64, t38709: f64, t38711: f64, t38713: f64, t38717: f64, t38721: f64, t38723: f64, t38727: f64) -> f64 {
    let t38729 = -0.12579236915841660827e-2_f64 * t33831 - 0.42874018118069736972e-3_f64 * t38701 + 0.10718504529517434243e-2_f64 * t38704 - t33840 + 0.85748036236139473944e-3_f64 * t38706 - t33842 + t33844 + t33852 + 0.41930789719472202757e-3_f64 * t33853 + 0.80031500487063509015e-2_f64 * t38709 - 0.94344276868812456204e-3_f64 * t38711 + t36823 + 7.0_f64 / 72.0_f64 * t38713 + 0.18868855373762491241e-2_f64 * t38717 + 0.94344276868812456205e-2_f64 * t38721 - 0.31448092289604152068e-3_f64 * t38723 - 0.20965394859736101379e-3_f64 * t38727;
    t38729
}
