//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1033/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1033(t30314: f64, t30319: f64, t2304: f64, t7610: f64, t1988: f64, t8561: f64, t30316: f64, t34171: f64, t34173: f64, t34176: f64, t34179: f64, t34183: f64, t34189: f64, t34193: f64, t34197: f64, t34201: f64, t34204: f64, t34208: f64, t34210: f64, t34211: f64) -> f64 {
    let t34212 = 0.7640625e-2_f64 * t30314;
    let t34214 = 0.16006300097412701803e-1_f64 * t30319;
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    let t34218 = 0.62896184579208304136e-3_f64 * t34217;
    let t34219 = -t34171 + t34173 + t34176 - 0.10482697429868050689e-2_f64 * t34179 - 0.47172138434406228102e-2_f64 * t34183 - 0.62896184579208304136e-2_f64 * t34189 + 0.18868855373762491241e-2_f64 * t34193 + 0.15724046144802076034e-2_f64 * t34197 - 0.23586069217203114051e-2_f64 * t34201 - 0.80031500487063509015e-2_f64 * t34204 - 0.37737710747524982482e-2_f64 * t34208 - t34210 - t34211 - t34212 - 0.31448092289604152068e-3_f64 * t30316 + t34214 - 0.31448092289604152068e-3_f64 * t34215 - t34218;
    t34219
}
