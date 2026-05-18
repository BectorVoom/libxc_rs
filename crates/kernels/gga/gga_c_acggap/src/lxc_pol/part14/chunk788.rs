//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 788/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk788<F: Float>(t1588: F, t2001: F, t1988: F, t2327: F, t1487: F, t6: F, t422: F, t599: F, t598: F, t1982: F, t1983: F, t507: F) -> (F, F, F, F, F, F) {
    let t8849 = t2001 * t1588;
    let t8851 = t1988 * t2327;
    let t8853 = t6 * t1487;
    let t8855 = t422 * t8853 * t599;
    let t8856 = t598 * t8855;
    let t8859 = t1982 * t507 * t1983;
    (t8849, t8851, t8853, t8855, t8856, t8859)
}
