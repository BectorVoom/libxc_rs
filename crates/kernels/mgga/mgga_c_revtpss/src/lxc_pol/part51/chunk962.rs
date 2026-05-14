//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 962/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk962<F: Float>(t13426: F, t8460: F, t18227: F, t27123: F, t28219: F, t125363: F, t125366: F, t125368: F, t125370: F, t125372: F, t125374: F, t125377: F, t125379: F, t125381: F, t125383: F, t125211: F, t125213: F, t125215: F, t125217: F, t125223: F, t125343: F, t125361: F, t1310: F, t1453: F, t1843: F, t1911: F, t2007: F, t28160: F, t32161: F, t32179: F, t33578: F, t33580: F, t33583: F, t33630: F, t33647: F, t33903: F, t508: F, t5517: F, t569: F, t649: F, t7221: F, t7725: F, t8447: F) -> (F,) {
    let t125384 = t13426 * t8460;
    let t125385 = 2.0 * t125384;
    let t125386 = t18227 * t8460;
    let t125387 = 2.0 * t125386;
    let t125388 = t27123 * t8460;
    let t125389 = 2.0 * t125388;
    let t125390 = t28219 * t8460;
    let t125391 = 2.0 * t125390;
    let t125392 = 4.0 * t125363 + 4.0 * t125366 + 4.0 * t125368 + 4.0 * t125370 + 4.0 * t125372 + 4.0 * t125374 + t125377 + t125379 + t125381 + t125383 + t125385 + t125387 + t125389 + t125391;
    let t125401 = -t125211 - t125213 - t125215 - t125217 - t33578 - t33580 - t33583 - 2.0 * t28160 * t2007 - 2.0 * t7725 * t7221 - t649 * t33903 + 2.0 * t125223 + (t125361 + t125392) * t569 + t32179 * t1911 + t33647 * t1453 - t32161 * t1843 - t8447 * t5517 - t125343 * t508 - t33630 * t1310;
    (t125401,)
}
