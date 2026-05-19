//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 839/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk839<F: Float>(t218: F, t2693: F, t760: F, t715: F, t902: F, t2801: F, t2803: F, t761: F, t771: F, t229: F, t2825: F, t2809: F, t780: F) -> (F, F, F, F, F) {
    let t11756 = F::new(8.0) * t760 * t2693 * t218;
    let t11762 = t715 * t902;
    let t11770 = F::cast_from(0.3103560775156404018e4_f64) * t2801 * t761 * t2803 * t771;
    let t11772 = F::new(16.0) * t229 * t2825;
    let t11775 = F::cast_from(0.57895126195293126241e3_f64) * t2809 * t780 * t771;
    (t11756, t11762, t11770, t11772, t11775)
}
