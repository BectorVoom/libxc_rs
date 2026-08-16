//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2192/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2192(t4194: f64, t5398: f64, t607: f64, t750: f64, t32: f64, t5519: f64, t2517: f64, t707: f64, t16616: f64, t2535: f64, t16701: f64, t2427: f64) -> (f64, f64, f64, f64, f64) {
    let t57965 = t4194 * t750 * t5398 * t607;
    let t57973 = t32 * t5519;
    let t57992 = t707 * t2517 * t5398;
    let t58021 = t16616 * t2535;
    let t58047 = t2427 * t16701;
    (t57965, t57973, t57992, t58021, t58047)
}
