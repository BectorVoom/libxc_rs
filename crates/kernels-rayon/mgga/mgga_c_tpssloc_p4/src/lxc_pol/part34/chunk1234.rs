//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1234/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1234(t105519: f64, t105698: f64, t1492: f64, t2054: f64, t21053: f64, t25168: f64, t259: f64, t26728: f64, t29040: f64, t67305: f64, t67339: f64, t87898: f64, t87915: f64, t99003: f64, t99022: f64, t99036: f64) -> f64 {
    let t108448 = -3.0_f64 * t67305 * t2054 + 0.11514538467937585055e0_f64 * t99003 + 0.19739208802178717238e0_f64 * t105519 - 3.0_f64 * t67339 * t2054 - 0.29608813203268075857e0_f64 * t105698 - 0.24674011002723396548e-1_f64 * t99022 - 0.15626873635058151147e0_f64 * t87898 - 0.49348022005446793095e-1_f64 * t87915 + 3.0_f64 * t1492 * t29040 * t259 + 0.9869604401089358619e-1_f64 * t99036 - 18.0_f64 * t25168 * t26728 * t21053;
    t108448
}
