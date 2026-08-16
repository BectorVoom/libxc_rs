//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3312/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3312(t23059: f64, t4147: f64, t23087: f64, t9593: f64, t566: f64, t6836: f64, t198: f64, t21969: f64, t40076: f64, t40079: f64, t4139: f64, t47152: f64, t48327: f64, t48330: f64, t48332: f64, t48334: f64, t5532: f64, t5591: f64, t85993: f64, t85994: f64) -> (f64, f64, f64) {
    let t86825 = t23059 * t4147;
    let t86828 = t23087 * t9593;
    let t86839 = t6836 * t566;
    let t86846 = 18.0_f64 * t198 * t5591 * t86839 + 9.0_f64 * t21969 * t4139 * t5532 + t40076 - t40079 + t47152 - t48327 - t48330 + t48332 - t48334 - t85993 + t85994;
    (t86825, t86828, t86846)
}
