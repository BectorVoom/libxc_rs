//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1224/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1224(t81153: f64, t1887: f64, t22797: f64, t22715: f64, t6887: f64, t12225: f64, t22641: f64, t268: f64, t547: f64, t6559: f64, t12248: f64, t2006: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81154 = 0.98696044010893586188e-1_f64 * t81153;
    let t81159 = t22797 * t1887;
    let t81186 = t22715 * t6887;
    let t81195 = t22641 * t12225;
    let t81228 = t6559 * t547 * t268;
    let t81243 = t12248 * t2006;
    (t81154, t81159, t81186, t81195, t81228, t81243)
}
