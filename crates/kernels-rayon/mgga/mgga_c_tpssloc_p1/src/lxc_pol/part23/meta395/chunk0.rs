//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1200/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1200(t2517: f64, t5520: f64, t12945: f64, t4205: f64, t32: f64, t5519: f64, t5398: f64, t707: f64, t16616: f64, t2535: f64, t2371: f64, t41115: f64, t5593: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57897 = t5520 * t2517;
    let t57960 = t4205 * t12945;
    let t57973 = t32 * t5519;
    let t57992 = t707 * t2517 * t5398;
    let t58021 = t16616 * t2535;
    let t58057 = t16616 * t2371;
    let t58421 = t41115 * t5593;
    (t57897, t57960, t57973, t57992, t58021, t58057, t58421)
}
