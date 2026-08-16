//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1471/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1471(t121226: f64, t121228: f64, t121231: f64, t121233: f64, t121234: f64, t121237: f64, t12725: f64, t24932: f64, t26898: f64, t27163: f64, t27290: f64, t27863: f64, t27888: f64, t33690: f64, t7042: f64, t7057: f64, t7061: f64, t7266: f64, t7796: f64, t8690: f64, t8835: f64) -> f64 {
    let t124969 = -2.0_f64 * t12725 * t8835 - 2.0_f64 * t24932 * t7796 + 3.0_f64 * t26898 * t8690 - 2.0_f64 * t27163 * t7266 - 2.0_f64 * t27290 * t7042 - 2.0_f64 * t27863 * t7061 - 2.0_f64 * t27888 * t7796 - 2.0_f64 * t33690 * t7057 - t121226 - t121228 - t121231 - t121233 - t121234 - t121237;
    t124969
}
