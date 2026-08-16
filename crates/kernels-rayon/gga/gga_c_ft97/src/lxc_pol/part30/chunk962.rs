//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 962/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk962(t34311: f64, t92: f64, t1466: f64, t34057: f64, t681: f64, t2399: f64, t7613: f64, t28658: f64, t7203: f64, t2691: f64, t33939: f64, t4113: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142663 = t34311 * t92;
    let t142677 = t1466 * t681 * t34057;
    let t142688 = 2.0_f64 / 27.0_f64 * t1466 * t2399 * t7613;
    let t142696 = t28658 * t7203;
    let t142697 = t2691 * t142696;
    let t142704 = t4113 * t33939 * t7203;
    (t142663, t142677, t142688, t142696, t142697, t142704)
}
