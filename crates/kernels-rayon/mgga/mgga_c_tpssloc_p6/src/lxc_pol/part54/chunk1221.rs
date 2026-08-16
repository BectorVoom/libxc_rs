//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1221/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1221(t191: f64, t192: f64, t7900: f64, t2020: f64, t7754: f64, t8607: f64, t7940: f64, t8643: f64, t1983: f64, t25224: f64, t8547: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33363 = t7900 * t191 * t192;
    let t33364 = t33363 * t2020;
    let t33365 = t8607 * t7754;
    let t33366 = t7940 * t8643;
    let t33367 = t1983 * t33366;
    let t33371 = t25224 * t8547;
    let t33372 = t1880 * t33371;
    (t33363, t33364, t33365, t33366, t33367, t33371, t33372)
}
