//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2311/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2311<F: Float>(t1509: F, t661: F, t26: F, t65: F, t1651: F, t385: F, t1078: F, t1695: F, t1774: F, t494: F, t1276: F, t1828: F) -> (F, F, F, F, F, F) {
    let t31443 = t1509 * t661;
    let t33127 = F::new(1.0) / t65 / t26;
    let t33754 = t385 * t1651;
    let t33768 = t1078 * t1695;
    let t34934 = t494 * t1774;
    let t34964 = t1276 * t1828;
    (t31443, t33127, t33754, t33768, t34934, t34964)
}
