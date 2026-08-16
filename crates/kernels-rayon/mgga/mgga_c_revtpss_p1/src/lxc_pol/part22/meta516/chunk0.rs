//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2281/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2281(t16840: f64, t3436: f64, t12238: f64, t1733: f64, t3379: f64, t5105: f64, t12327: f64, t1723: f64, t3391: f64, t12331: f64, t3390: f64, t5079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16842 = 0.16081979498692535067e2_f64 * t16840 * t3436;
    let t16844 = 1.0_f64 * t12238 * t1733;
    let t16846 = 2.0_f64 * t3379 * t5105;
    let t16851 = t12327 * t1723;
    let t16852 = t16851 * t3391;
    let t16854 = t12331 * t1723;
    let t16855 = t16854 * t3391;
    let t16857 = t3390 * t5079;
    (t16842, t16844, t16846, t16851, t16852, t16854, t16855, t16857)
}
