//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2809/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2809(t10535: f64, t136: f64, t2457: f64, t4424: f64, t10523: f64, t14568: f64, t2482: f64, t2801: f64, t4423: f64, t879: f64, t14606: f64, t1568: f64, t2722: f64) -> (f64, f64, f64, f64, f64) {
    let t51614 = t10535 * t4424 * t136 * t2457;
    let t51615 = 0.34697458558045176417e-2_f64 * t51614;
    let t51617 = t14568 * t10523;
    let t51621 = t2482 * t879 * t4423 * t2801;
    let t51623 = t14606 * t10523;
    let t51625 = t1568 * t2722;
    (t51615, t51617, t51621, t51623, t51625)
}
