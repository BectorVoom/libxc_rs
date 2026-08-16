//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1356/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1356<F: Float>(t1006: F, t1692: F, t1812: F, t18247: F, t18254: F, t18271: F, t20012: F, t20018: F, t20510: F, t20514: F, t2439: F, t33: F, t5671: F, t5678: F, t5853: F, t62610: F, t6354: F, t64896: F, t64982: F, t65030: F, t66235: F, t66262: F, t66281: F, t66311: F, t66317: F, t66604: F) -> F {
    let t66796 = -t1692 * t20514 * t18271 / F::cast_from(2.0_f64) - t1692 * t5853 * t65030 / F::cast_from(2.0_f64) - t66235 + t1692 * t20510 * t1006 - F::cast_from(3.0_f64) * t62610 * t20018 + F::cast_from(6.0_f64) * t66311 * t20012 - t66262 - F::cast_from(3.0_f64) * t66317 * t18247 - t1692 * t66281 * t5678 + F::cast_from(3.0_f64) * t2439 * t1812 * t64896 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2439 * t6354 * t18254 - t1692 * t5853 * t64982 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2439 * t20510 * t5671 + t1692 * t66604 * t33 / F::cast_from(2.0_f64);
    t66796
}
