//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 699/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk699(t1882: f64, t794: f64, t6562: f64, t225: f64, t857: f64, t1887: f64, t206: f64, t6546: f64) -> (f64, f64, f64, f64) {
    let t6563 = t794 * t1882;
    let t6564 = t6562 * t6563;
    let t6565 = 0.41123351671205660912e-2_f64 * t6564;
    let t6571 = t225 * t857;
    let t6579 = t6546 * t206 * t1887;
    (t6563, t6565, t6571, t6579)
}
