//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 567/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk567<F: Float>(t3400: F, t1996: F, t2000: F, t2044: F, t2047: F, t4560: F, t4581: F, t4604: F, t4606: F, t4607: F, t4608: F, t1761: F, t1787: F, t1795: F, t1799: F, t1834: F, t1838: F, t1873: F, t1966: F, t1968: F, t1985: F, t1992: F, t2050: F) -> (F, F) {
    let t4634 = F::new(2.0) * t3400;
    let t4635 = t4604 - t4606 - t4607 - t4608 - t4560 + t4634 + t4581 - t1996 - t2000 + t2044 + t2047;
    let t4636 = t2050 - t1834 + t1992 - t1838 + t1985 + t1873 - t1968 - t1966 - t1761 + t1799 + t1787 + t1795;
    (t4635, t4636)
}
