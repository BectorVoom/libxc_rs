//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2820/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2820<F: Float>(t10489: F, t11054: F, t11084: F, t1940: F, t198: F, t207: F, t2403: F, t39989: F, t4343: F, t4541: F, t4542: F, t4556: F, t50106: F, t50114: F, t50115: F, t50151: F, t50190: F, t50216: F, t50250: F, t50276: F, t50853: F, t50857: F, t51218: F, t51253: F, t51723: F, t51762: F, t765: F, t892: F) -> F {
    let t51769 = -F::cast_from(9.0_f64) * t2403 * t11084 * t4343 + t50106 - t39989 + F::cast_from(6.0_f64) * t4541 * t4542 * t10489 - t1940 * t4556 * t11054 + t50114 + t50115 + F::cast_from(3.0_f64) * t198 * t765 * t50151 + t198 * t207 * (t50190 + t50216 + t50250 + t50276 + t51218 + t51253 + t51723 + t51762) * t892 - t50853 - t50857;
    t51769
}
