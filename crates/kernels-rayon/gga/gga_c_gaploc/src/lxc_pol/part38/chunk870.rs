//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 870/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk870(t40612: f64, t40614: f64, t40620: f64, t40630: f64, t40632: f64, t40634: f64, t43072: f64, t43073: f64, t44855: f64, t44857: f64, t44858: f64, t739: f64) -> (f64, f64) {
    let t44860 = 7.0_f64 / 256.0_f64 * t40612;
    let t44861 = 63.0_f64 / 8192.0_f64 * t40614;
    let t44862 = 63.0_f64 / 524288.0_f64 * t40620;
    let t44863 = 21.0_f64 / 524288.0_f64 * t40630;
    let t44864 = 21.0_f64 / 8192.0_f64 * t40632;
    let t44865 = 7.0_f64 / 768.0_f64 * t40634;
    let t44866 = t44855 - t44857 + t44858 / 2.0_f64 + t43072 - t43073 + t44860 + t44861 - t44862 + t44863 - t44864 - t44865;
    let t44874 = t739 * t44866;
    (t44866, t44874)
}
