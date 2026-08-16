//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1122/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1122(t2312: f64, t9087: f64, t20065: f64, t2321: f64, t9074: f64, t1358: f64, t9075: f64, t2300: f64, t6295: f64, t6525: f64, t2317: f64, t6541: f64) -> (f64, f64, f64, f64, f64) {
    let t29852 = 0.47425011059460249332e-2_f64 * t2312 * t9087;
    let t29860 = 0.23712505529730124666e-2_f64 * t9074 * t20065 * t2321;
    let t29862 = 0.63233348079280332442e-2_f64 * t1358 * t9075;
    let t29865 = 0.23712505529730124666e-2_f64 * t6525 * t2300 * t6295;
    let t29868 = 0.47425011059460249332e-2_f64 * t6525 * t6541 * t2317;
    (t29852, t29860, t29862, t29865, t29868)
}
