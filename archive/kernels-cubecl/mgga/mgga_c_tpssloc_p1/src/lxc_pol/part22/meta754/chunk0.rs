//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2533/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2533<F: Float>(t449: F, t71289: F, t71308: F, t1671: F, t63750: F, t18686: F, t4782: F, t14845: F, t6021: F, t18835: F, t4740: F, t11310: F, t11350: F, t1136: F, t1155: F, t15171: F, t15225: F, t18612: F, t18616: F, t18640: F, t18786: F, t4835: F, t51382: F, t51389: F, t51727: F, t6052: F, t6084: F, t71095: F, t71097: F, t71217: F, t71245: F) -> (F, F, F, F, F, F) {
    let t71310 = (t71289 + t71308) * t449;
    let t71313 = F::cast_from(3.0_f64) * t63750 * t1671;
    let t71315 = F::cast_from(3.0_f64) * t18686 * t4782;
    let t71317 = F::cast_from(3.0_f64) * t14845 * t6021;
    let t71319 = F::cast_from(3.0_f64) * t4740 * t18835;
    let t71322 = -t71095 + F::cast_from(0.10526802520742363173e2_f64) * t51389 * t18612 + t71097 + F::cast_from(0.62071215503128080361e4_f64) * t11350 * t6052 * t15171 * t1136 + F::cast_from(18.0_f64) * t51382 * t18640 + F::cast_from(0.30762056574649219973e4_f64) * t11310 * t6084 * t15225 * t1155 + t71217 - F::cast_from(0.31168546390226634766e3_f64) * t51727 * t18616 - F::cast_from(0.19751673498613801407e-1_f64) * t71310 - t71245 - t71313 - t71315 - t71317 - t71319 + F::cast_from(0.17544670867903938621e1_f64) * t4835 * t18786;
    (t71310, t71313, t71315, t71317, t71319, t71322)
}
