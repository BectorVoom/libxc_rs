//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1133/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1133(t12857: f64, t1588: f64, t12856: f64, t609: f64, t625: f64, t4313: f64, t4390: f64, t4413: f64, t4473: f64, t4479: f64, t12938: f64, t629: f64, t632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40484 = t1588 * t12857;
    let t40512 = t609 / t12856 / t625;
    let t40514 = t4313 * t4313;
    let t40515 = 1.0_f64 / t40514;
    let t40541 = t4390 * t4413;
    let t40556 = t4473 * t4479;
    let t40653 = t629 / t12938 / t632;
    (t40484, t40512, t40515, t40541, t40556, t40653)
}
