//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 386/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk386<F: Float>(t154: F, t2559: F, t222: F, t2563: F, t805: F, t119: F, t2379: F, t210: F, t2553: F, t225: F, t2591: F) -> (F, F, F, F, F, F) {
    let t2600 = t2559 * t154;
    let t2602 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2605 = t119 * t2379;
    let t2606 = t210 * t2605;
    let t2610 = t210 * t119 * t2553;
    let t2613 = t2591 * t225;
    (t2600, t2602, t2603, t2606, t2610, t2613)
}
