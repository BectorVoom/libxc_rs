//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 888/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk888(t225: f64, t22624: f64, t22622: f64, t214: f64, t3879: f64, t1887: f64, t22797: f64, t2006: f64, t3850: f64, t268: f64, t547: f64, t6559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80699 = t22624 * t225;
    let t80704 = t22622 * t225;
    let t80707 = t214 * t3879;
    let t81159 = t22797 * t1887;
    let t81203 = t2006 * t3850;
    let t81228 = t6559 * t547 * t268;
    (t80699, t80704, t80707, t81159, t81203, t81228)
}
