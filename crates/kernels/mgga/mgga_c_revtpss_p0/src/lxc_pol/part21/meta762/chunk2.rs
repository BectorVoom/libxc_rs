//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2704/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2704<F: Float>(t2349: F, t656: F, t10227: F, t97: F, t10241: F, t105: F, t4273: F, t588: F, t10228: F, t10242: F, t13472: F, t13475: F, t13476: F, t13485: F, t13496: F, t1504: F, t1509: F, t2255: F, t2256: F, t2350: F, t2358: F, t2362: F, t31283: F, t31443: F, t46196: F, t46212: F, t580: F, t658: F, t661: F, t9342: F) -> F {
    let t49774 = t656 * t2349;
    let t49777 = t97 * t10227;
    let t49787 = t105 * t10241;
    let t49804 = F::new(20.0) * t97 * t4273 * t588;
    let t49809 = F::new(50.0) / F::new(27.0) * t656 * t13472 + F::new(25.0) * t656 * t13485 - F::new(10.0) / F::new(3.0) * t13496 * t2255 * t2362 - F::new(10.0) * t13475 * t9342 * t658 + F::new(10.0) * t13496 * t9342 * t661 - F::new(100.0) / F::new(9.0) * t49774 * t13476 - F::new(10.0) / F::new(9.0) * t49777 * t31283 * t2256 - F::new(10.0) / F::new(9.0) * t49777 * t2255 * t2350 + F::new(10.0) / F::new(3.0) * t13475 * t2255 * t2256 - F::new(10.0) / F::new(9.0) * t49787 * t31443 * t2362 + F::new(10.0) / F::new(9.0) * t49787 * t2255 * t2358 + F::new(40.0) / F::new(81.0) * t97 * t46196 * t1504 * t10228 + F::new(10.0) / F::new(3.0) * t97 * t2349 * t580 * t658 + t49804 + F::new(40.0) / F::new(81.0) * t105 * t46212 * t1509 * t10242;
    t49809
}
