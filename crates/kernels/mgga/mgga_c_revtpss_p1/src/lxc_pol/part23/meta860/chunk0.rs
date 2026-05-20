//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2750/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2750<F: Float>(t21342: F, t460: F, t1276: F, t6587: F, t487: F, t70208: F, t1269: F, t20849: F, t1770: F, t5412: F, t3555: F, t6695: F) -> (F, F, F, F, F, F) {
    let t72959 = t460 * t21342;
    let t73051 = t1276 * t6587;
    let t73055 = t70208 * t487;
    let t73137 = t20849 * t1269;
    let t73187 = t1770 * t5412;
    let t73205 = t3555 * t6695;
    (t72959, t73051, t73055, t73137, t73187, t73205)
}
