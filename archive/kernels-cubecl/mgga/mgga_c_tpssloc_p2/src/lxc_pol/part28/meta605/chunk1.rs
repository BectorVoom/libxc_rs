//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1912/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1912<F: Float>(t16044: F, t1992: F, t6976: F, t1372: F, t1799: F, t1307: F, t26331: F, t26446: F, t26411: F, t6914: F, t12420: F, t5335: F) -> (F, F, F, F, F) {
    let t90752 = t1992 * t6976 * t16044;
    let t90754 = t1372 * t1799;
    let t90757 = t26331 * t26446 * t90754 * t1307;
    let t90759 = t6914 * t26411;
    let t90763 = t26331 * t6976 * t5335 * t12420;
    (t90752, t90754, t90757, t90759, t90763)
}
