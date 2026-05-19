//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 966/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk966<F: Float>(t10802: F, t5559: F, t1052: F, t2728: F, t1960: F, t1022: F, t830: F, t1: F, t787: F, t2631: F, t2628: F, t2976: F) -> (F, F, F, F, F, F, F) {
    let t10804 = F::new(6.0) * t5559 * t10802;
    let t10805 = t1052 * t2728;
    let t10807 = F::new(2.0) * t1960 * t10805;
    let t10809 = t830 * t1022;
    let t10810 = t10809 * t1;
    let t10811 = t787 * t10810;
    let t10813 = F::cast_from(0.42900587942220512003e1_f64) * t10811 * t2631;
    let t10814 = t2976 * t2628;
    (t10804, t10805, t10807, t10809, t10811, t10813, t10814)
}
