//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2586/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2586(t51993: f64, t52047: f64, t52094: f64, t52150: f64, t52197: f64, t52257: f64, t52303: f64, t52374: f64, t15814: f64, t225: f64, t11720: f64, t1751: f64) -> (f64, f64, f64) {
    let t52377 = t51993 + t52047 + t52094 + t52150 + t52197 + t52257 + t52303 + t52374;
    let t52386 = t15814 * t225;
    let t52424 = t1751 * t11720;
    (t52377, t52386, t52424)
}
