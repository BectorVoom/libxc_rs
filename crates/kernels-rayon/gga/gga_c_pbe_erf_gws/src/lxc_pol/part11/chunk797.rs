//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 797/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk797(t3644: f64, t967: f64, t100: f64, t10050: f64, t10053: f64, t1563: f64, t10: f64, t10069: f64, t10072: f64, t10075: f64, t10078: f64, t496: f64, t5749: f64, t5751: f64, t5753: f64, t5755: f64, t5759: f64, t5776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12898 = t3644 * t967;
    let t12899 = t12898 * t100;
    let t12906 = 0.2923025e1_f64 * t10050;
    let t12907 = 0.14615125e1_f64 * t10053;
    let t12912 = t1563 * t12898;
    let t12913 = t10 * t12912;
    let t12916 = -t5749 - t5751 + t5753 - t5755 - t5759 - t12906 + t12907 - 3.0_f64 / 2.0_f64 * t10069 + t10072 / 2.0_f64 - 0.881424e1_f64 * t10075 + 0.220356e1_f64 * t10078 - t5776 - 6.0_f64 * t496 * t12913;
    (t12898, t12899, t12906, t12907, t12912, t12913, t12916)
}
