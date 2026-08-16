//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1133/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1133(t15585: f64, t3068: f64, t1114: f64, t5072: f64, t1101: f64, t13335: f64, t926: f64, t15232: f64, t15355: f64, t15361: f64, t15363: f64, t15365: f64, t15411: f64, t15413: f64, t15417: f64, t15421: f64, t15426: f64, t15443: f64, t15446: f64, t15448: f64, t15465: f64, t15467: f64, t15473: f64, t15475: f64) -> (f64, f64, f64, f64) {
    let t15586 = t3068 * t15585;
    let t15589 = t5072 * t1114;
    let t15590 = t3068 * t15589;
    let t15595 = t1101 * t13335;
    let t15596 = t926 * t15595;
    let t15599 = -t15232 - t15355 + t15361 - t15363 + t15365 + t15411 + t15413 - t15417 + t15421 - t15426 + t15443 + t15446 + t15448 - t15465 - t15467 - t15473 - t15475;
    (t15586, t15590, t15596, t15599)
}
