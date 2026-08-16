//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3107/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3107(t1160: f64, t24453: f64, t1170: f64, t12423: f64, t12481: f64, t24411: f64, t24431: f64, t24436: f64, t45174: f64, t58307: f64, t58336: f64, t6487: f64, t6519: f64, t81148: f64, t81150: f64, t81152: f64, t81252: f64, t81307: f64, t81352: f64, t81558: f64, t81560: f64, t81562: f64) -> f64 {
    let t81791 = t24453 * t1160;
    let t81796 = -0.19751673498613801407e-1_f64 * t81252 - t81148 + t81150 - t81152 - 6.0_f64 * t58336 * t6487 + 6.0_f64 * t12423 * t24431 - 0.35089341735807877242e1_f64 * t58307 * t6519 + 0.35089341735807877242e1_f64 * t12481 * t24436 + 1.0_f64 * t81791 * t1170 + 0.10254018858216406658e4_f64 * t45174 * t24411 + t81307 - t81352 - t81558 - t81560 + t81562;
    t81796
}
