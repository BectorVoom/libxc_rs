//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1091/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1091<F: Float>(t125211: F, t125213: F, t125215: F, t125217: F, t125223: F, t125343: F, t125361: F, t125392: F, t1310: F, t1453: F, t1843: F, t1911: F, t2007: F, t28160: F, t32161: F, t32179: F, t33578: F, t33580: F, t33583: F, t33630: F, t33647: F, t33903: F, t508: F, t5517: F, t569: F, t649: F, t7221: F, t7725: F, t8447: F) -> F {
    let t125401 = -t125211 - t125213 - t125215 - t125217 - t33578 - t33580 - t33583 - F::cast_from(2.0_f64) * t28160 * t2007 - F::cast_from(2.0_f64) * t7725 * t7221 - t649 * t33903 + F::cast_from(2.0_f64) * t125223 + (t125361 + t125392) * t569 + t32179 * t1911 + t33647 * t1453 - t32161 * t1843 - t8447 * t5517 - t125343 * t508 - t33630 * t1310;
    t125401
}
