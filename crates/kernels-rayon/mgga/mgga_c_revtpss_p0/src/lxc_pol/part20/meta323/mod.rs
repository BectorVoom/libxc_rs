//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1234;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta323(t12948: f64, t3610: f64, t1263: f64, t3584: f64, t1122: f64, t1042: f64, t1260: f64, t3666: f64, t3172: f64, t3713: f64, t3711: f64, t127: f64, t3661: f64, t371: f64, t1235: f64, t12640: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12949, t12951, t12952, t12953, t12956) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1234(t12948, t3610, t1263, t3584, t1122, t1042, t1260, t3666);
        let (t12959, t12960, t12963, t12964, t12966) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1235(t3172, t3713, t3711, t127, t3661, t371, t1235, t12640, t225);
    (t12949, t12951, t12952, t12953, t12956, t12959, t12960, t12963, t12964, t12966)
}
