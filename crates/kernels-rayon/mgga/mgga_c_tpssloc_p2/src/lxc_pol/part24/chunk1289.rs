//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1289/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1289(t1385: f64, t1992: f64, t22635: f64, t3886: f64, t3911: f64, t22649: f64, t6883: f64, t1372: f64, t212: f64, t22642: f64, t6890: f64, t1985: f64, t22666: f64, t22934: f64) -> (f64, f64, f64, f64) {
    let t81305 = t1992 * t22635 * t3886 * t1385 * t3911;
    let t81307 = t6883 * t22649;
    let t81311 = t22642 * t212 * t1372 * t6890;
    let t81315 = t1985 * t22666 * t22934;
    (t81305, t81307, t81311, t81315)
}
