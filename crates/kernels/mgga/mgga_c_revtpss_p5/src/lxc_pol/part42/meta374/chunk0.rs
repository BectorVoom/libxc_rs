//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1227/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1227<F: Float>(t6075: F, t892: F, t262: F, t5962: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t14353: F, t14433: F, t1544: F, t18557: F, t18558: F, t18561: F, t18564: F, t18565: F, t18567: F, t2403: F, t2404: F, t4541: F, t775: F, t9514: F, t9517: F, t9521: F) -> F {
    let t18850 = t6075 * t892;
    let t18860 = t262 * t5962;
    let t18864 = F::new(6.0) * t14353 * t1544 * t2403 + F::new(3.0) * t18850 * t2403 * t775 + F::new(6.0) * t18860 * t4541 * t775 + F::new(3.0) * t2403 * t2404 * t5962 - t10568 + t10577 + t10582 - t10584 - t10586 + t14433 - t18557 + t18558 + t18561 - t18564 + t18565 + t18567 + t9514 - t9517 - t9521;
    t18864
}
