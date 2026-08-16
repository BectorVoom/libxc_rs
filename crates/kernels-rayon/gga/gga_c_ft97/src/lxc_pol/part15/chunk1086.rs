//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1086/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1086(t4417: f64, t4668: f64, t12680: f64, t13208: f64, t17198: f64, t1901: f64, t20764: f64, t20768: f64, t2210: f64, t3434: f64, t3440: f64, t41269: f64, t4454: f64, t4458: f64, t4733: f64, t77196: f64, t77198: f64, t77214: f64, t85401: f64, t87009: f64, t9133: f64, t9144: f64) -> (f64, f64) {
    let t87534 = t4417 * t4668;
    let t87552 = 8.0_f64 / 3.0_f64 * t1901 * t9144 * t4458 * t4733 - 8.0_f64 / 3.0_f64 * t1901 * t13208 * t87009 - 8.0_f64 / 9.0_f64 * t1901 * t41269 * t4454 * t4733 - 4.0_f64 / 3.0_f64 * t1901 * t2210 * t17198 * t4458 + 8.0_f64 / 3.0_f64 * t1901 * t9133 * t3434 * t87534 + 4.0_f64 / 3.0_f64 * t1901 * t12680 * t20764 + 8.0_f64 / 3.0_f64 * t1901 * t12680 * t20768 - 4.0_f64 * t1901 * t2210 * t3440 * t85401 + 8.0_f64 / 27.0_f64 * t77196 + 8.0_f64 / 9.0_f64 * t77198 - 4.0_f64 / 9.0_f64 * t77214;
    (t87534, t87552)
}
