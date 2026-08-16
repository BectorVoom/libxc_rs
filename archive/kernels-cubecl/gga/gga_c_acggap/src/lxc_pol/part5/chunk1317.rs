//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1317/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1317<F: Float>(t11756: F, t11770: F, t1734: F, t19981: F, t19982: F, t19984: F, t19986: F, t19988: F, t19989: F, t19990: F, t19991: F, t5412: F, t5506: F, t694: F, t695: F, t96: F) -> F {
    let t24571 = F::cast_from(6.0_f64) * t1734 * t5412 * t96 + F::cast_from(6.0_f64) * t5506 * t694 * t695 - t11756 + t11770 + t19981 + t19982 - t19984 + t19986 - t19988 - t19989 - t19990 - t19991;
    t24571
}
