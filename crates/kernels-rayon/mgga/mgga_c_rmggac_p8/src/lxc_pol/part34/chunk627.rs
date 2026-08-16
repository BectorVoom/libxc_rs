//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 627/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk627(t15616: f64, t656: f64, t2145: f64, t15297: f64, t2265: f64, t2415: f64, t2010: f64, t615: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15617 = t15616 * t656;
    let t15618 = t2145 * t15617;
    let t15619 = 0.34093327067806677161e-2_f64 * t15618;
    let t15620 = 0.1276937996798935182e-4_f64 * t15297;
    let t15621 = t2415 * t2265;
    let t15622 = t2010 * t15621;
    let t15623 = 0.36021158228745895953e-3_f64 * t15622;
    let t15624 = t698 * t615;
    (t15617, t15619, t15620, t15621, t15623, t15624)
}
