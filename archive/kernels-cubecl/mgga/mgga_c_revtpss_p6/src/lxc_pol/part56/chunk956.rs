//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 956/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk956<F: Float>(t1453: F, t32101: F, t32102: F, t32107: F, t32109: F, t32112: F, t32116: F, t32823: F, t32824: F, t32840: F, t32843: F, t32845: F, t32849: F, t32850: F, t33343: F, t33346: F, t33381: F, t569: F, t651: F, t671: F, t7586: F, t7591: F, t8463: F, t8967: F) -> F {
    let t33384 = t1453 * t8967 - F::cast_from(2.0_f64) * t33343 * t651 - F::cast_from(2.0_f64) * t33346 * t671 + t33381 * t569 - F::cast_from(4.0_f64) * t7586 * t7591 + t32101 - t32102 - t32107 - t32109 - t32112 - t32116 + F::cast_from(2.0_f64) * t32823 + F::cast_from(2.0_f64) * t32824 - F::cast_from(4.0_f64) * t32840 - F::cast_from(4.0_f64) * t32843 - F::cast_from(4.0_f64) * t32845 - F::cast_from(2.0_f64) * t32849 + F::cast_from(6.0_f64) * t32850 - t8463;
    t33384
}
