//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2540/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2540<F: Float>(t10704: F, t4395: F, t2904: F, t4446: F, t10523: F, t1573: F, t10629: F, t1556: F, t2842: F, t10702: F, t10828: F, t1580: F) -> (F, F, F, F, F, F, F) {
    let t49072 = t4395 * t10704;
    let t49096 = t4446 * t2904;
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    let t49226 = t2842 * t1556;
    let t49240 = t10702 * t1556;
    let t49263 = t10828 * t1580;
    (t49072, t49096, t49099, t49104, t49226, t49240, t49263)
}
