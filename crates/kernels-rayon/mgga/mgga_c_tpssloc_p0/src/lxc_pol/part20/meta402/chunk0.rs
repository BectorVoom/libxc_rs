//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1799/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1799(t13779: f64, t4343: f64, t2986: f64, t134: f64, t2978: f64, t344: f64) -> (f64, f64, f64, f64) {
    let t13780 = t13779 * t4343;
    let t13782 = 0.37037037037037037036e-3_f64 * t2986 * t13780;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    (t13780, t13782, t13783, t13784)
}
