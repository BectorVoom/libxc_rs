//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 775/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk775<F: Float>(t432: F, t7274: F, t1852: F, t452: F, t32420: F, t83: F, t5644: F, t5710: F, t1901: F, t32496: F, t32500: F, t32504: F, t32508: F, t32510: F, t32512: F, t32517: F, t32520: F, t32524: F, t446: F) -> (F, F, F, F, F) {
    let t32527 = t7274 * t432;
    let t32529 = t452 * t1852 * t32527;
    let t32532 = t83 * t32420;
    let t32536 = t452 * t5710 * t5644;
    let t32539 = -F::new(2.0) / F::new(9.0) * t1901 * t32496 - F::new(2.0) / F::new(3.0) * t446 * t32500 + F::new(2.0) / F::new(3.0) * t446 * t32504 - t32508 + t32510 - F::new(2.0) / F::new(3.0) * t446 * t32512 + t1901 * t32517 / F::new(9.0) + F::new(4.0) / F::new(3.0) * t446 * t32520 + F::new(2.0) / F::new(3.0) * t446 * t32524 - F::new(2.0) / F::new(3.0) * t446 * t32529 - F::new(2.0) * t446 * t32532 + F::new(2.0) / F::new(3.0) * t446 * t32536;
    (t32527, t32529, t32532, t32536, t32539)
}
