//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1234/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1234(t12948: f64, t3610: f64, t1263: f64, t3584: f64, t1122: f64, t1042: f64, t1260: f64, t3666: f64) -> (f64, f64, f64, f64, f64) {
    let t12949 = t3610 * t12948;
    let t12951 = t1263 * t3584;
    let t12952 = t12951 * t1122;
    let t12953 = t1042 * t12952;
    let t12956 = t3666 * t1260;
    (t12949, t12951, t12952, t12953, t12956)
}
