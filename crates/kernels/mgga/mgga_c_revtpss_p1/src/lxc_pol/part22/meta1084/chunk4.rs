//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3929/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3929<F: Float>(t1507: F, t2357: F, t10227: F, t10241: F, t105: F, t13493: F, t13497: F, t13500: F, t13506: F, t21835: F, t21845: F, t21846: F, t21850: F, t21860: F, t2255: F, t2256: F, t2349: F, t2350: F, t2358: F, t2362: F, t31283: F, t31443: F, t4269: F, t4279: F, t46212: F, t49777: F, t49787: F, t49804: F, t580: F, t5823: F, t5907: F, t5911: F, t656: F, t658: F, t97: F) -> F {
    let t75625 = t1507 * t2357;
    let t75634 = -F::new(10.0) / F::new(27.0) * t97 * t21835 * t2256 + F::new(20.0) / F::new(9.0) * t97 * t4269 * t580 - F::new(10.0) / F::new(27.0) * t97 * t10227 * t5823 * t2350 + F::new(100.0) / F::new(81.0) * t1507 * t13493 - F::new(50.0) / F::new(3.0) * t1507 * t13506 + F::new(40.0) / F::new(81.0) * t105 * t46212 * t5907 * t2358 - F::new(20.0) / F::new(9.0) * t105 * t4279 * t580 - F::new(10.0) / F::new(27.0) * t105 * t10241 * t5911 * t2358 - F::new(100.0) / F::new(27.0) * t656 * t21846 + F::new(20.0) / F::new(9.0) * t97 * t2349 * t21850 * t658 + F::new(10.0) / F::new(9.0) * t97 * t21845 * t2256 - F::new(100.0) / F::new(27.0) * t1507 * t13500 - F::new(10.0) / F::new(27.0) * t105 * t21860 * t2362 + F::new(200.0) / F::new(27.0) * t75625 * t13497 + t49804 - F::new(40.0) / F::new(27.0) * t49777 * t31283 * t2255 + F::new(40.0) / F::new(27.0) * t49787 * t31443 * t2255;
    t75634
}
