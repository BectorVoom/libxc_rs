//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1742/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1742(t26361: f64, t225: f64, t7919: f64, t2085: f64, t5210: f64, t1824: f64) -> (f64, f64, f64, f64) {
    let t27067 = 0.38381794893125283518e-1_f64 * t26361;
    let t27068 = t7919 * t225;
    let t27070 = t5210 * t2085;
    let t27074 = t2085 * t1824;
    (t27067, t27068, t27070, t27074)
}
