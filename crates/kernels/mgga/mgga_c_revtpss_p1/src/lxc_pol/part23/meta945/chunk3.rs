//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3107/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3107<F: Float>(t1160: F, t24453: F, t1170: F, t12423: F, t12481: F, t24411: F, t24431: F, t24436: F, t45174: F, t58307: F, t58336: F, t6487: F, t6519: F, t81148: F, t81150: F, t81152: F, t81252: F, t81307: F, t81352: F, t81558: F, t81560: F, t81562: F) -> F {
    let t81791 = t24453 * t1160;
    let t81796 = -F::cast_from(0.19751673498613801407e-1_f64) * t81252 - t81148 + t81150 - t81152 - F::new(6.0) * t58336 * t6487 + F::new(6.0) * t12423 * t24431 - F::cast_from(0.35089341735807877242e1_f64) * t58307 * t6519 + F::cast_from(0.35089341735807877242e1_f64) * t12481 * t24436 + F::new(1.0) * t81791 * t1170 + F::cast_from(0.10254018858216406658e4_f64) * t45174 * t24411 + t81307 - t81352 - t81558 - t81560 + t81562;
    t81796
}
