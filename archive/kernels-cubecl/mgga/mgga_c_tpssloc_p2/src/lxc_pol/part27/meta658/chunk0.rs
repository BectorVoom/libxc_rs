//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2299/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2299<F: Float>(t26411: F, t6914: F, t12420: F, t26331: F, t5335: F, t6976: F, t1351: F, t1992: F, t5318: F, t550: F, t16036: F, t22633: F, t3807: F) -> (F, F, F, F) {
    let t90759 = t6914 * t26411;
    let t90760 = F::cast_from(0.38381794893125283518e-1_f64) * t90759;
    let t90763 = t26331 * t6976 * t5335 * t12420;
    let t90770 = t1992 * t6976 * t5318 * t1351 * t550;
    let t90774 = t22633 * t6976 * t16036 * t3807;
    (t90760, t90763, t90770, t90774)
}
