//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 982/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk982(t10495: f64, t10537: f64, t10747: f64, t10751: f64, t45: f64, t2865: f64, t3605: f64, t730: f64, t10513: f64, t5528: f64, t10518: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10753 = t10495 + t10537 + t10747 + t10751;
    let t10754 = t45 * t10753;
    let t10755 = t2865 * t3605;
    let t10757 = 0.35089341735807877242e1_f64 * t730 * t10755;
    let t10760 = t5528 * t10513;
    let t10764 = t652 * t10518;
    (t10753, t10754, t10755, t10757, t10760, t10764)
}
