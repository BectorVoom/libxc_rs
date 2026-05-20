//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3921/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3921<F: Float>(t21969: F, t566: F, t1353: F, t13600: F, t22486: F, t3889: F, t39989: F, t4139: F, t47086: F, t47088: F, t5536: F, t5591: F, t6836: F, t74121: F, t74122: F, t74123: F, t74124: F, t74125: F, t9599: F) -> F {
    let t75379 = t566 * t21969;
    let t75386 = F::new(12.0) * t1353 * t5536 * t75379 + F::new(12.0) * t13600 * t4139 * t5591 + F::new(6.0) * t22486 * t3889 * t5536 - F::new(6.0) * t5536 * t6836 * t9599 - t39989 - t47086 + t47088 - t74121 + t74122 + t74123 + t74124 - t74125;
    t75386
}
