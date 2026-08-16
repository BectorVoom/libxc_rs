//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1059/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1059(t12948: f64, t3610: f64, t1263: f64, t3584: f64, t1122: f64, t1042: f64, t1260: f64, t3666: f64, t3172: f64, t3713: f64, t3711: f64, t127: f64, t3661: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12949 = t3610 * t12948;
    let t12951 = t1263 * t3584;
    let t12952 = t12951 * t1122;
    let t12953 = t1042 * t12952;
    let t12956 = t3666 * t1260;
    let t12959 = t3172 * t3713;
    let t12960 = t3711 * t12959;
    let t12963 = t371 * t127 * t3661;
    (t12949, t12953, t12956, t12959, t12960, t12963)
}
