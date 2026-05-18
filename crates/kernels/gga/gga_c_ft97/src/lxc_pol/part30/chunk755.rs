//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 755/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk755<F: Float>(t33517: F, t33530: F, t258: F, t1403: F, t33245: F, t33248: F, t33255: F, t33259: F, t33264: F, t33269: F, t33272: F, t33275: F, t33279: F, t33490: F, t33496: F, t33499: F, t33504: F, t5996: F, t6002: F, t6005: F, t6011: F, t6064: F, t6068: F, t7437: F, t7491: F) -> (F, F, F) {
    let t33531 = t33517 + t33530;
    let t33532 = t33531 * t258;
    let t33534 = t1403 * t33245 - F::new(2.0) / F::new(3.0) * t1403 * t33248 - t7437 * t6011 / F::new(3.0) - t1403 * t33255 / F::new(3.0) + t1403 * t33259 / F::new(3.0) + t7437 * t6068 / F::new(6.0) - F::new(4.0) * t33264 + t5996 * t7491 / F::new(3.0) + t1403 * t33269 / F::new(3.0) - F::new(4.0) * t33272 - F::new(2.0) * t33275 - F::new(2.0) / F::new(3.0) * t1403 * t33279 - F::new(2.0) * t33490 + t7437 * t6064 / F::new(6.0) + t6002 * t33496 / F::new(9.0) - t33499 * t6005 / F::new(18.0) - t6002 * t33504 / F::new(9.0) + F::new(2.0) * t33532;
    (t33531, t33532, t33534)
}
