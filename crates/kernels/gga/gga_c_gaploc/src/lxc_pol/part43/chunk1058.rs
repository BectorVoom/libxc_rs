//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1058/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1058<F: Float>(t41143: F, t43653: F, t43658: F, t43661: F, t43664: F, t43670: F, t43674: F, t43680: F, t43682: F, t47280: F, t47283: F, t47286: F, t47290: F, t47296: F, t47299: F, t47303: F, t47306: F, t47309: F, t47315: F, t47317: F) -> F {
    let t51120 = t43653 - t47280 - t47283 + F::new(0.76685851907841499353e0) * t41143 + t43658 + t43661 + t43664 + F::new(0.23005755572352449806e2) * t47286 + t47290 - t43670 - t43674 - t43680 + t43682 - F::new(0.14300195980740170668e1) * t47296 + F::new(0.92023022289409799224e1) * t47299 - t47303 + t47306 + t47309 + t47315 + t47317;
    t51120
}
