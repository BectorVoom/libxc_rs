//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1298/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1298<F: Float>(t6077: F, t62306: F, t18646: F, t6080: F, t65442: F, t65444: F, t116: F, t20287: F, t20217: F, t508: F, t20319: F, t1665: F, t5960: F) -> (F, F, F, F, F, F, F, F) {
    let t67510 = t62306 * t6077;
    let t67512 = t6080 * t18646;
    let t67532 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t65442;
    let t67533 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t65444;
    let t67541 = t20287 * t116;
    let t67782 = t508 * t20217;
    let t67816 = t116 * t20319;
    let t67849 = F::cast_from(2.0_f64) * t1665 * t5960;
    (t67510, t67512, t67532, t67533, t67541, t67782, t67816, t67849)
}
