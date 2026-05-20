//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1947/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1947<F: Float>(t101172: F, t101176: F, t101182: F, t101187: F, t101190: F, t101193: F, t101357: F, t2048: F, t26187: F, t28105: F, t28109: F, t28112: F, t7343: F, t7352: F, t7706: F, t95255: F, t95259: F) -> F {
    let t101849 = F::new(176.0) / F::new(27.0) * t95255 - F::new(2.0) / F::new(3.0) * t101357 * t2048 - F::new(5.0) / F::new(3.0) * t95259 * t7706 - F::new(10.0) / F::new(3.0) * t26187 * t28105 - F::new(10.0) / F::new(3.0) * t26187 * t28109 - F::new(5.0) / F::new(3.0) * t7343 * t101172 - F::new(10.0) / F::new(3.0) * t7343 * t101176 - F::new(5.0) / F::new(3.0) * t7343 * t101182 - F::new(2.0) / F::new(3.0) * t101187 * t2048 - F::new(4.0) / F::new(3.0) * t101190 * t2048 - F::new(4.0) / F::new(3.0) * t101193 * t2048 - F::new(4.0) / F::new(3.0) * t28112 * t7352;
    t101849
}
