//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1617/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1617(t18316: f64, t18337: f64, t18390: f64, t18951: f64, t18989: f64, t19029: f64, t19075: f64, t19117: f64, t466: f64, t5068: f64, t6260: f64, t18940: f64, t491: f64) -> (f64, f64, f64, f64) {
    let t19120 = t18316 + t18337 + t18390 + t18951 + t18989 + t19029 + t19075 + t19117;
    let t19121 = t466 * t19120;
    let t19123 = t6260 * t5068;
    let t19128 = t491 * t18940;
    (t19120, t19121, t19123, t19128)
}
