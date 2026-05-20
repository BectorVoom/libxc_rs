//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1032/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1032<F: Float>(t7313: F, t8568: F, t32171: F, t508: F, t1310: F, t8454: F, t2042: F, t7324: F, t2040: F, t7331: F, t7334: F, t1459: F, t8611: F) -> (F, F, F, F, F, F, F) {
    let t32329 = t8568 * t7313;
    let t32338 = F::new(2.0) * t32171 * t508;
    let t32340 = F::new(2.0) * t8454 * t1310;
    let t32358 = t7324 * t2042;
    let t32360 = t2040 * t7331;
    let t32362 = t2040 * t7334;
    let t32365 = F::new(6.0) * t1459 * t8611;
    (t32329, t32338, t32340, t32358, t32360, t32362, t32365)
}
