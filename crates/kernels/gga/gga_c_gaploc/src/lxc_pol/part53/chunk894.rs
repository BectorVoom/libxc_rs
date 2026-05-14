//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 894/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk894<F: Float>(t43028: F, t43032: F, t43040: F, t43043: F, t43049: F, t43053: F, t43054: F, t43055: F, t43094: F, t47629: F, t47631: F, t47634: F, t47636: F, t47640: F, t47644: F, t47646: F, t47650: F, t47652: F) -> (F,) {
    let t51013 = t43028 + t43032 - t43040 - t47629 + t47631 - t47634 + t47636 + t43043 - t43049 - t43053 + t43054 - t43055 + 0.20508069947045931423e-1 * t47640 + t47644 + 0.46143157380853345702e-1 * t47646 - 0.30762104920568897134e-1 * t47650 + 0.85450291446024714264e-3 * t47652 + t43094;
    (t51013,)
}
