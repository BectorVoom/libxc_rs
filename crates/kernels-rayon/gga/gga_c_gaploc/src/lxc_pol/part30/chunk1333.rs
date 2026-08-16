//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1333/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1333(t11116: f64, t22274: f64, t11069: f64, t5662: f64, t11016: f64, t8478: f64, t8638: f64, t29052: f64, t3025: f64, t2714: f64, t8556: f64, t3040: f64, t7593: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33851 = 0.1853729108614466568e0_f64 * t22274 * t11116;
    let t33853 = 0.1022478025437886658e1_f64 * t5662 * t11069;
    let t33857 = 0.14300195980740170668e1_f64 * t8478 * t11016;
    let t33859 = 0.14300195980740170668e1_f64 * t8638 * t11016;
    let t33861 = 0.14300195980740170668e1_f64 * t3025 * t29052;
    let t33863 = 0.47667319935800568892e0_f64 * t2714 * t8556;
    let t33865 = 0.35750489951850426669e0_f64 * t7593 * t3040;
    (t33851, t33853, t33857, t33859, t33861, t33863, t33865)
}
