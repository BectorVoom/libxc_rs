//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 872/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk872<F: Float>(t11778: F, t61: F, t121: F, t3584: F, t1229: F, t676: F, t486: F, t11552: F, t221: F, t456: F, t1176: F, t3242: F) -> (F, F, F, F, F, F, F) {
    let t11779 = t61 * t11778;
    let t11784 = t121 * t3584;
    let t11789 = t676 * t1229;
    let t11818 = t676 * t486;
    let t11832 = t221 * t11552;
    let t11834 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t456 * t11832;
    let t11848 = t1176 * t3242;
    (t11779, t11784, t11789, t11818, t11832, t11834, t11848)
}
