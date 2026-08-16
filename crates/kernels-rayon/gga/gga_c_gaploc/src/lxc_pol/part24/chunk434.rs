//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 434/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk434(t2066: f64, t774: f64, t769: f64, t779: f64, t1836: f64, t531: f64, t1865: f64, t808: f64, t568: f64, t836: f64, t321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2067 = t2066 * t774;
    let t2070 = t769 * t779;
    let t2073 = t531 * t1836;
    let t2076 = t808 * t1865;
    let t2077 = t568 * t2076;
    let t2080 = t836 * t1865;
    let t2081 = t568 * t2080;
    let t2084 = t321 * t321;
    (t2067, t2070, t2073, t2077, t2081, t2084)
}
