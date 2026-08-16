//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1210/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1210(t81688: f64, t81716: f64, t24269: f64, t24278: f64, t2679: f64, t2684: f64, t7101: f64, t808: f64, t812: f64, t81656: f64, t81661: f64, t81667: f64, t81670: f64, t81675: f64, t81691: f64, t81695: f64, t81697: f64, t81702: f64, t81704: f64, t81709: f64, t81713: f64, t9958: f64) -> f64 {
    let t84995 = 0.27415567780803773942e-2_f64 * t81688;
    let t85003 = 0.19739208802178717238e0_f64 * t81716;
    let t85007 = -t812 * t7101 * t9958 - 3.0_f64 * t812 * t24269 * t2679 + 0.9869604401089358619e-1_f64 * t81656 - 0.9869604401089358619e-1_f64 * t81661 + 3.0_f64 * t808 * t24278 - 0.49348022005446793095e-1_f64 * t81667 + 0.49348022005446793095e-1_f64 * t81670 - 0.16449340668482264365e-1_f64 * t81675 - t84995 + 0.24674011002723396548e-1_f64 * t81691 + 0.29608813203268075857e0_f64 * t81695 + 0.11514538467937585055e0_f64 * t81697 - 0.49348022005446793095e-1_f64 * t81702 + 0.11514538467937585055e0_f64 * t81704 - 0.49348022005446793095e-1_f64 * t81709 + 0.9869604401089358619e-1_f64 * t81713 + t85003 - 3.0_f64 * t812 * t24269 * t2684;
    t85007
}
