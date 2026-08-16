//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 249/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk249(t304: f64, t305: f64, t1771: f64, t303: f64, t1775: f64, t849: f64, t458: f64, t854: f64, t2344: f64, t295: f64) -> (f64, f64, f64, f64, f64) {
    let t2755 = 1.0_f64 / t305 / t304;
    let t2761 = 4.0_f64 / 9.0_f64 * t1771 * t303;
    let t2762 = t1775 * t849;
    let t2764 = t458 * t854;
    let t2766 = t2344 * t295;
    (t2755, t2761, t2762, t2764, t2766)
}
