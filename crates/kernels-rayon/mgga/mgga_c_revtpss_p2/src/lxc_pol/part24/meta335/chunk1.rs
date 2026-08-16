//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1168/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1168(t10698: f64, t23114: f64, t828: f64, t23148: f64, t855: f64, t10824: f64, t10826: f64, t10885: f64, t18459: f64, t18475: f64, t18485: f64, t18487: f64, t18491: f64, t18518: f64, t18532: f64, t18623: f64, t18644: f64, t851: f64) -> (f64, f64, f64) {
    let t23342 = t10698 * t828 * t23114;
    let t23346 = t855 * t828 * t23148;
    let t23357 = 0.30011812682648815881e-2_f64 * t18459 - 0.25724410870841842183e-1_f64 * t851 * t23342 - 0.85748036236139473944e-3_f64 * t851 * t23346 - 0.60023625365297631762e-1_f64 * t18475 + 0.12004725073059526352e-1_f64 * t18485 - t10824 + t10826 - 0.60023625365297631762e-2_f64 * t18487 + 0.30011812682648815881e-2_f64 * t18491 - t10885 - 0.24009450146119052704e-1_f64 * t18518 - 0.38115002106963996168e-4_f64 * t18532 - 0.38115002106963996168e-4_f64 * t18623 + 0.30492001685571196935e-3_f64 * t18644;
    (t23342, t23346, t23357)
}
