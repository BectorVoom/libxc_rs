//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1122/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1122(t26361: f64, t225: f64, t7919: f64, t2085: f64, t5210: f64, t1824: f64, t5250: f64, t1352: f64, t26393: f64, t1825: f64, t24116: f64, t26406: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27067 = 0.38381794893125283518e-1_f64 * t26361;
    let t27068 = t7919 * t225;
    let t27070 = t5210 * t2085;
    let t27074 = t2085 * t1824;
    let t27075 = t27074 * t5250;
    let t27078 = t27074 * t1352;
    let t27082 = 0.16449340668482264365e-1_f64 * t26393;
    let t27086 = t24116 * t1825;
    let t27088 = 0.38381794893125283518e-1_f64 * t26406;
    (t27067, t27068, t27070, t27074, t27075, t27078, t27082, t27086, t27088)
}
