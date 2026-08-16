//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1435/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1435<F: Float>(t1671: F, t71877: F, t18686: F, t6021: F, t6024: F, t63755: F, t21810: F, t4740: F, t21813: F, t51120: F, t1164: F, t6088: F, t64537: F) -> (F, F, F, F, F, F) {
    let t78327 = F::cast_from(4.0_f64) * t71877 * t1671;
    let t78329 = F::cast_from(6.0_f64) * t18686 * t6021;
    let t78331 = F::cast_from(0.96491876992155210402e2_f64) * t63755 * t6024;
    let t78333 = F::cast_from(4.0_f64) * t4740 * t21810;
    let t78335 = F::cast_from(0.2069040516770936012e4_f64) * t51120 * t21813;
    let t78338 = F::cast_from(0.62337092780453269531e3_f64) * t1164 * t64537 * t6088;
    (t78327, t78329, t78331, t78333, t78335, t78338)
}
