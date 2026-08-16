//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1635/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1635<F: Float>(t1544: F, t18268: F, t2403: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t40088: F, t40099: F, t40103: F, t4541: F, t5962: F, t77341: F, t87650: F, t87651: F) -> F {
    let t87951 = F::cast_from(24.0_f64) * t1544 * t4541 * t77341 - F::cast_from(18.0_f64) * t18268 * t2403 * t5962 + t39799 + t39807 - t39813 - t39818 - t39823 + t40084 + t40088 + t40099 + t40103 + t87650 + t87651;
    t87951
}
