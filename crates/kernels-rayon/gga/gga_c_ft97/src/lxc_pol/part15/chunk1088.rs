//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1088/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1088(t1060: f64, t12703: f64, t13153: f64, t13220: f64, t144: f64, t17198: f64, t1901: f64, t20035: f64, t20902: f64, t20930: f64, t2179: f64, t2185: f64, t3439: f64, t4454: f64, t4458: f64, t446: f64, t4462: f64, t4668: f64, t4724: f64, t4839: f64, t569: f64, t77383: f64, t77386: f64, t86973: f64, t87163: f64, t925: f64) -> f64 {
    let t87657 = -2.0_f64 / 3.0_f64 * t446 * t569 * t4839 * t4462 + 8.0_f64 * t446 * t2185 * t2179 * t4668 * t4724 + 4.0_f64 / 3.0_f64 * t446 * t569 * t4839 * t4458 - 8.0_f64 / 3.0_f64 * t446 * t569 * t1060 * t20035 - t446 * t144 * t87163 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t77383 - 8.0_f64 / 3.0_f64 * t1901 * t12703 * t86973 - 8.0_f64 / 3.0_f64 * t1901 * t13220 * t20902 * t925 + 4.0_f64 / 9.0_f64 * t1901 * t3439 * t17198 * t4454 - 8.0_f64 / 3.0_f64 * t1901 * t13153 * t20930 + 4.0_f64 / 9.0_f64 * t77386;
    t87657
}
