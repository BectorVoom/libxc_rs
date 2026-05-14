//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 618/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk618<F: Float>(t3113: F, t984: F, t12383: F, t12386: F, t12397: F, t12400: F, t12412: F, t12783: F, t12784: F) -> (F,) {
    let t12785 = t984 * t3113;
    let t12787 = 9.0 / 256.0 * t12383;
    let t12788 = 9.0 / 8192.0 * t12386;
    let t12789 = 3.0 / 8192.0 * t12397;
    let t12790 = 3.0 / 256.0 * t12400;
    let t12791 = 2.0 * t12412;
    let t12792 = t12783 + t12784 - t12785 / 2.0 - t12787 - t12788 + t12789 + t12790 + t12791;
    (t12792,)
}
