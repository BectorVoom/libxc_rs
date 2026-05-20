//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1405/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1405<F: Float>(t13334: F, t38: F, t1486: F, t2251: F, t2259: F, t4217: F, t607: F, t1471: F, t1487: F, t1494: F, t2252: F, t2260: F, t2263: F, t2312: F, t4196: F, t4218: F, t4238: F, t608: F, t641: F, t85: F) -> F {
    let t13335 = t38 * t13334;
    let t13340 = t2251 * t1486;
    let t13343 = t2259 * t1486;
    let t13346 = t607 * t4217;
    let t13363 = t13335 * t85 / F::new(24.0) - t1471 * t2312 / F::new(12.0) - t13340 * t85 / F::new(12.0) - t13343 * t85 / F::new(12.0) - t13346 * t85 / F::new(6.0) - t4196 * t641 / F::new(6.0) - t2252 * t1494 / F::new(12.0) - t2260 * t1494 / F::new(12.0) - t2263 * t1494 / F::new(6.0) - t608 * t4238 / F::new(6.0) + t4218 * t641 / F::new(12.0) + t1487 * t2312 / F::new(24.0);
    t13363
}
