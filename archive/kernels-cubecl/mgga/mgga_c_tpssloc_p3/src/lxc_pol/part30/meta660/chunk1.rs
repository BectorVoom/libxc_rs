//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2082/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2082<F: Float>(t90749: F, t1372: F, t1799: F, t26411: F, t6914: F, t22704: F, t22705: F, t5345: F, t22690: F, t552: F, t26447: F, t90607: F) -> (F, F, F, F, F, F) {
    let t90750 = F::cast_from(0.76763589786250567036e-1_f64) * t90749;
    let t90754 = t1372 * t1799;
    let t90759 = t6914 * t26411;
    let t90760 = F::cast_from(0.38381794893125283518e-1_f64) * t90759;
    let t90781 = t22704 * t22705 * t5345;
    let t90782 = F::cast_from(0.82246703342411321824e-2_f64) * t90781;
    let t90787 = t22690 * t552;
    let t90789 = t90607 * t90787 * t26447;
    (t90750, t90754, t90760, t90782, t90787, t90789)
}
