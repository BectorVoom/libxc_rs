//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 500/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk500(t1246: f64, t6260: f64, t3625: f64, t6252: f64, t493: f64, t6238: f64, t1244: f64, t1729: f64, t1756: f64, t1758: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t5064: f64, t6168: f64, t6253: f64, t6257: f64) -> f64 {
    let t6261 = t6260 * t1246;
    let t6263 = t6252 * t3625;
    let t6265 = t493 * t6238;
    let t6267 = 2.0_f64 * t1244 * t6257 + t1244 * t6261 + 2.0_f64 * t1729 * t1758 + 2.0_f64 * t1756 * t5064 + 2.0_f64 * t3610 * t6253 - t3624 * t6263 + t470 * t6265 + t494 * t6168;
    t6267
}
