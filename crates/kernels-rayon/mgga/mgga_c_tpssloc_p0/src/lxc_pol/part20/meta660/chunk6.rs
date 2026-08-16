//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2470/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2470(t10160: f64, t10182: f64, t1049: f64, t1052: f64, t1065: f64, t11085: f64, t13736: f64, t13939: f64, t14526: f64, t14545: f64, t14555: f64, t14658: f64, t1635: f64, t3026: f64, t3169: f64, t3174: f64, t3176: f64, t3206: f64, t349: f64, t388: f64, t43440: f64, t43619: f64, t4557: f64, t4693: f64, t4694: f64, t50457: f64, t990: f64) -> f64 {
    let t50744 = 6.0_f64 * t1052 * t1065 * t14658 * t3174 + 6.0_f64 * t1052 * t3174 * t3206 * t4693 + 3.0_f64 * t1049 * t13939 * t388 + 3.0_f64 * t14526 * t388 * t990 + t349 * t388 * t50457 - 6.0_f64 * t10160 * t4694 + 6.0_f64 * t10182 * t4557 - t11085 * t4557 - 18.0_f64 * t13736 * t3026 - 18.0_f64 * t13736 * t3169 + 6.0_f64 * t14545 * t3176 + 6.0_f64 * t14555 * t3176 - t1635 * t43440 - t1635 * t43619;
    t50744
}
