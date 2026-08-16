//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1101/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1101(t1887: f64, t22797: f64, t22715: f64, t6887: f64, t12225: f64, t22641: f64, t268: f64, t547: f64, t6559: f64, t22644: f64, t81152: f64, t1988: f64, t81071: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81159 = t22797 * t1887;
    let t81186 = t22715 * t6887;
    let t81195 = t22641 * t12225;
    let t81228 = t6559 * t547 * t268;
    let t81281 = t81152 * t22644;
    let t81317 = t81071 * t1988;
    (t81159, t81186, t81195, t81228, t81281, t81317)
}
