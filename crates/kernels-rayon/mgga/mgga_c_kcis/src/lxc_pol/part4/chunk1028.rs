//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1028/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1028(t1562: f64, t4354: f64, t592: f64, t4357: f64, t600: f64, t1347: f64, t3910: f64, t1341: f64, t3944: f64, t11388: f64, t473: f64, t11536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12729 = 1.0_f64 / t4354 / t1562;
    let t12730 = t592 * t12729;
    let t12732 = 1.0_f64 / t4357 / t600;
    let t12736 = t3910 * t1347;
    let t12741 = t1341 * t3944;
    let t12744 = t473 * t11388;
    let t12751 = t473 * t11536;
    (t12730, t12732, t12736, t12741, t12744, t12751)
}
