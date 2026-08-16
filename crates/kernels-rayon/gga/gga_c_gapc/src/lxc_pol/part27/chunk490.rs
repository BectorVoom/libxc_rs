//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 490/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk490(t2786: f64, t285: f64, t191: f64, t1775: f64, t315: f64, t331: f64) -> (f64, f64, f64, f64) {
    let t2787 = t2786 * t285;
    let t2788 = t2787 * t191;
    let t2795 = t1775 * t315;
    let t2800 = t331 * t331;
    let t2801 = 1.0_f64 / t2800;
    (t2787, t2788, t2795, t2801)
}
