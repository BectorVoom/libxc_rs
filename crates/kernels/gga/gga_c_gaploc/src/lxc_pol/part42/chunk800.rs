//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 800/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk800<F: Float>(t10736: F, t28412: F, t913: F, t2365: F, t33087: F, t8775: F, t10639: F, t10912: F, t787: F, t899: F, t13118: F, t15362: F) -> (F, F, F, F) {
    let t43432 = t28412 * t913 * t10736;
    let t43446 = t8775 * t2365 * t33087;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    let t43464 = t15362 * t13118;
    (t43432, t43446, t43454, t43464)
}
