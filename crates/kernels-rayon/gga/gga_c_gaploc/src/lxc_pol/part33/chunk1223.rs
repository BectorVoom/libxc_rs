//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1223/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1223(t10627: f64, t1858: f64, t787: f64, t107: f64, t548: f64, t734: f64, t2365: f64, t24474: f64, t7390: f64, t32514: f64, t7584: f64, t7585: f64) -> (f64, f64, f64, f64) {
    let t32743 = t1858 * t10627;
    let t32744 = t787 * t32743;
    let t32745 = t107 * t548;
    let t32748 = 0.79445533226334281486e-1_f64 * t32744 * t32745 * t734;
    let t32752 = t7390 * t2365 * t24474;
    let t32753 = 0.29792074959875355558e-1_f64 * t32752;
    let t32756 = 0.87421871174939309262e2_f64 * t7584 * t7585 * t32514;
    (t32745, t32748, t32753, t32756)
}
