//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1064/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1064(t121034: f64, t1385: f64, t1404: f64, t32276: f64, t32278: f64, t3985: f64, t8591: f64, t240: f64, t843: f64, t31752: f64, t32197: f64, t136: f64, t2457: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121035 = t121034 * t1385;
    let t121043 = t32276 * t1404 * t32278;
    let t121045 = t8591 * t3985;
    let t121046 = 0.49169913065300780973e-2_f64 * t121045;
    let t121056 = t1385 * t843 * t240;
    let t121057 = t31752 * t121056;
    let t121058 = t121057 * t32197;
    let t121072 = t545 * t136 * t2457;
    (t121035, t121043, t121046, t121057, t121058, t121072)
}
