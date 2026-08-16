//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1118/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1118(t23228: f64, t7479: f64, t81573: f64, t23012: f64, t7485: f64, t7489: f64, t25245: f64, t82031: f64, t7529: f64, t22690: f64, t7520: f64, t23030: f64, t25258: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86916 = t81573 * t23228 * t7479;
    let t86955 = t23012 * t7485;
    let t86991 = t23012 * t7489;
    let t87068 = t82031 * t25245;
    let t87080 = t23012 * t7529;
    let t87140 = t81573 * t22690 * t7520;
    let t87155 = t23030 * t25258;
    (t86916, t86955, t86991, t87068, t87080, t87140, t87155)
}
