//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1936/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1936(t17004: f64, t6581: f64, t16662: f64, t1894: f64, t236: f64, t6591: f64, t5568: f64, t81956: f64, t28389: f64, t81963: f64, t25068: f64, t4257: f64) -> (f64, f64, f64, f64, f64) {
    let t98703 = t6581 * t17004;
    let t98707 = t6591 * t1894 * t236 * t16662;
    let t98709 = t81956 * t5568;
    let t98711 = t81963 * t28389;
    let t98715 = t25068 * t4257;
    (t98703, t98707, t98709, t98711, t98715)
}
