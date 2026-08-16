//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2129/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2129<F: Float>(t26003: F, t7458: F, t26142: F, t4028: F, t22674: F, t28191: F, t80681: F, t1985: F, t22666: F, t28232: F, t26331: F, t26333: F, t90566: F) -> (F, F, F, F, F, F) {
    let t96842 = F::cast_from(4.0_f64) * t7458 * t26003;
    let t96844 = F::cast_from(4.0_f64) * t7458 * t26142;
    let t96846 = F::cast_from(4.0_f64) * t4028 * t26142;
    let t96848 = t80681 * t22674 * t28191;
    let t96851 = t1985 * t22666 * t28232;
    let t96854 = t26331 * t90566 * t26333;
    (t96842, t96844, t96846, t96848, t96851, t96854)
}
