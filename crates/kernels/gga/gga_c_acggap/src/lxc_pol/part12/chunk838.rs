//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 838/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk838<F: Float>(t2146: F, t2241: F, t464: F, t557: F, t8123: F, t8311: F, t8314: F, t8316: F, t8319: F, t8330: F, t8332: F, t8339: F, t9003: F, t9381: F, t9386: F, t9391: F, t9397: F, t9399: F) -> F {
    let t9401 = F::new(0.65854491829355115987e0) * t8123 - F::new(0.8673628188205199462e0) * t8311 + F::new(0.8673628188205199462e0) * t8314 - F::new(0.65854491829355115987e0) * t9381 + F::new(0.65854491829355115987e0) * t8319 + F::new(0.4336814094102599731e0) * t2146 * t9386 - F::new(0.65854491829355115987e0) * t8316 * t557 + t8330 - F::new(0.65854491829355115987e0) * t9391 * t464 - F::new(0.65854491829355115987e0) * t8332 + F::new(0.4336814094102599731e0) * t9003 * t2241 - t8339 + F::new(0.65854491829355115987e0) * t9397 - F::new(0.65854491829355115987e0) * t9399;
    t9401
}
