//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 753/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk753<F: Float>(t1424: F, t1454: F, t2354: F, t684: F, t33290: F, t33317: F, t33286: F, t33297: F, t33305: F, t33310: F, t33314: F, t33322: F, t33326: F, t33330: F, t33335: F) -> (F, F, F, F, F) {
    let t33502 = t1424 * t1454;
    let t33504 = t2354 * t33502 * t684;
    let t33508 = F::new(2.0) / F::new(9.0) * t33290;
    let t33513 = t33317 / F::new(9.0);
    let t33517 = t33286 / F::new(2.0) + t33508 + F::new(2.0) / F::new(9.0) * t33297 + F::new(4.0) / F::new(3.0) * t33305 - F::new(2.0) / F::new(3.0) * t33310 - t33314 / F::new(6.0) - t33513 - t33322 / F::new(9.0) - t33326 + F::new(2.0) / F::new(3.0) * t33330 + t33335 / F::new(12.0);
    (t33502, t33504, t33508, t33513, t33517)
}
