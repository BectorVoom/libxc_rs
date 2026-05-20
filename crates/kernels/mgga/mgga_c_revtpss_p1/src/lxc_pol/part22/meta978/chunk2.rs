//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3289/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3289<F: Float>(t14633: F, t14648: F, t14659: F, t14749: F, t1553: F, t1555: F, t18612: F, t225: F, t227: F, t229: F, t2634: F, t2638: F, t2639: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t61234: F, t61519: F, t62259: F, t62260: F, t62262: F, t62263: F, t62266: F, t62267: F, t62287: F, t62313: F, t73: F, t830: F, t832: F) -> F {
    let t62347 = -(t62259 + t62260 + t62262 + t62263 + t62266 + t62267 + t62287 + t62313) * t225 * t229 + F::new(6.0) * t830 * t18612 - F::new(12.0) * t2634 * t6010 - F::new(24.0) * t227 * t2638 * t61234 - F::new(12.0) * t6006 * t2639 + F::new(12.0) * t4409 * t4420 + F::new(3.0) * t2634 * t6013 + F::new(6.0) * t1553 * t14659 + F::new(3.0) * t227 * t832 * t61519 + F::new(240.0) * t4415 * t14648 * t14749 + F::new(3.0) * t6006 * t2642 + F::new(6.0) * t14633 * t1555 - F::new(48.0) * t4409 * t73 * t4417;
    t62347
}
