//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1786/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1786(t1012: f64, t1222: f64, t1225: f64, t1782: f64, t21213: f64, t21306: f64, t24736: f64, t24821: f64, t24827: f64, t24831: f64, t24836: f64, t3699: f64, t44348: f64, t44919: f64, t5373: f64, t57707: f64, t6653: f64, t83962: f64, t87107: f64, t87126: f64, t87145: f64) -> f64 {
    let t91119 = -0.25724410870841842184e-2_f64 * t21306 * t24736 + t1222 * t1012 * t44348 * t87145 / 6.0_f64 + 28.0_f64 / 243.0_f64 * t5373 * t24827 + 22.0_f64 / 81.0_f64 * t21213 * t6653 - 8.0_f64 / 27.0_f64 * t5373 * t24831 + 0.27439371595564631662e-1_f64 * t57707 * t24836 + 2.0_f64 / 9.0_f64 * t5373 * t24821 - t1222 * t1012 * t1225 * t87126 / 288.0_f64 - t1222 * t1012 * t44919 * t87145 / 12.0_f64 + t1222 * t1012 * t3699 * t87107 / 72.0_f64 + 154.0_f64 / 243.0_f64 * t83962 * t1782;
    t91119
}
