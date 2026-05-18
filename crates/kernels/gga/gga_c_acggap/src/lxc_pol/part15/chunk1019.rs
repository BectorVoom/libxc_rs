//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1019/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1019<F: Float>(t31603: F, t1588: F, t7614: F, t1988: F, t8855: F, t7799: F, t8859: F, t1488: F, t1980: F, t1982: F, t1983: F, t30318: F, t537: F) -> (F, F, F, F, F, F) {
    let t35812 = F::new(13.0) / F::new(144.0) * t31603;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35818 = t7799 * t8859;
    let t35827 = t1980 * t1982 * t1488 * t1983;
    let t35829 = t30318 * t537;
    (t35812, t35814, t35816, t35818, t35827, t35829)
}
