//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 563/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk563<F: Float>(t10820: F, t7573: F, t7427: F, t2013: F, t3489: F, t123: F, t2925: F, t883: F) -> (F, F, F, F) {
    let t10821 = t7573 * t10820;
    let t10823 = F::new(0.62115540045351614476e2) * t7427 * t10821;
    let t10824 = t2013 * t3489;
    let t10825 = F::new(0.19171462976960374838e0) * t10824;
    let t10826 = t2925 * t123;
    let t10827 = t10826 * t883;
    (t10823, t10824, t10825, t10827)
}
