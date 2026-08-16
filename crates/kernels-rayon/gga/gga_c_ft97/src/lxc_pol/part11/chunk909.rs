//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 909/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk909(t1853: f64, t38652: f64, t1820: f64, t8418: f64, t480: f64, t8417: f64, t8419: f64, t8466: f64, t8501: f64, t1841: f64, t1851: f64, t8213: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38653 = t1853 * t1853;
    let t38654 = t38652 * t38653;
    let t38657 = t8418 * t1853 * t1820;
    let t38659 = t480 * t8417;
    let t38660 = t38659 * t8419;
    let t38662 = t8466 * t8501;
    let t38664 = t1841 * t1851;
    let t38665 = t38664 * t1853;
    let t38675 = t8392 * t8213;
    (t38654, t38657, t38660, t38662, t38665, t38675)
}
