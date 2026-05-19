//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1153/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1153<F: Float>(t31603: F, t1588: F, t7614: F, t1988: F, t8855: F, t7799: F, t8859: F, t422: F, t4875: F, t598: F, t599: F, t6: F) -> (F, F, F, F, F) {
    let t35812 = F::new(13.0) / F::new(144.0) * t31603;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35817 = F::cast_from(0.21437009059034868486e-3_f64) * t35816;
    let t35818 = t7799 * t8859;
    let t35823 = t598 * t422 * t6 * t4875 * t599;
    (t35812, t35814, t35817, t35818, t35823)
}
