//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1399/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1399<F: Float>(t5946: F, t193: F, t3216: F, t336: F, t4700: F, t5950: F, t60874: F, t77157: F, t77159: F, t77224: F, t77226: F, t77229: F, t77232: F, t77236: F, t77470: F, t77474: F, t77478: F, t77482: F) -> F {
    let t77924 = t5946 * t5946;
    let t77929 = -F::cast_from(3.0_f64) * t193 * t3216 * t336 * t77924 + F::cast_from(12.0_f64) * t4700 * t5950 * t60874 + t77157 + t77159 - t77224 + t77226 - t77229 - t77232 + t77236 - t77470 + t77474 - t77478 - t77482;
    t77929
}
