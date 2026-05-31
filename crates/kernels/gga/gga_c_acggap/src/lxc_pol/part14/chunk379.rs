//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 379/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk379<F: Float>(t150: F, t1825: F, t1826: F, t1713: F, t921: F, t1734: F, t402: F, t153: F, t155: F, t519: F, t521: F) -> (F, F, F, F) {
    let t1828 = (t1825 + t1826) * t150;
    let t1832 = t921 * t1713;
    let t1835 = t402 * t1734;
    let t1838 = -F::cast_from(12.0_f64) * t153 * t1832 + F::cast_from(3.0_f64) * t153 * t1835 - t155 * t1828 + F::cast_from(6.0_f64) * t519 * t521;
    (t1828, t1832, t1835, t1838)
}
