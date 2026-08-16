//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1656/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1656(t3471: f64, t1169: f64, t1170: f64, t1188: f64, t12418: f64, t12423: f64, t12429: f64, t12431: f64, t12514: f64, t12555: f64, t12556: f64, t3452: f64, t3454: f64, t3472: f64, t3477: f64, t3479: f64, t3496: f64, t3521: f64, t3523: f64, t43750: f64, t43753: f64, t43966: f64, t44014: f64, t44021: f64, t44087: f64, t45057: f64, t45174: f64, t45177: f64, t45181: f64, t45188: f64, t45190: f64, t45194: f64, t45197: f64) -> f64 {
    let t45205 = t3471 * t3471;
    let t45218 = 0.4101607543286562663e4_f64 * t45174 * t12556 - 0.12304822629859687989e5_f64 * t45177 * t43753 * t12555 + 4.0_f64 * t45181 * t1170 + 6.0_f64 * t12418 * t3472 + 0.91082604192152556044e5_f64 * t45188 * t43753 * t45190 - 12.0_f64 * t45194 * t3454 - 0.77193501593724168322e3_f64 * t45197 * t12431 + 24.0_f64 * t12423 * t12514 - 24.0_f64 * t12429 * t45057 * t1169 - 6.0_f64 * t3452 * t45205 * t1169 + 0.96491876992155210402e2_f64 * t3477 * t45205 * t3479 + t43750 - 0.35089341735807877242e1_f64 * t3496 * t43966 * t1188 + 0.51947577317044391277e2_f64 * t3521 * t43966 * t3523 - t44014 + t44021 - t44087;
    t45218
}
