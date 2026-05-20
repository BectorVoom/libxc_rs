//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2882/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2882<F: Float>(t5962: F, t890: F, t1544: F, t18850: F, t18860: F, t18865: F, t2403: F, t27375: F, t4343: F, t4433: F, t4541: F, t4556: F, t50866: F, t63146: F, t77012: F, t77013: F, t77014: F, t77015: F, t77020: F) -> F {
    let t77425 = t5962 * t890;
    let t77429 = F::new(18.0) * t1544 * t4541 * t63146 + F::new(18.0) * t18850 * t4433 * t4541 + F::new(18.0) * t18860 * t4343 * t4541 - F::new(9.0) * t18865 * t2403 * t27375 - F::new(9.0) * t2403 * t4556 * t77425 + t50866 - t77012 - t77013 - t77014 + t77015 + t77020;
    t77429
}
