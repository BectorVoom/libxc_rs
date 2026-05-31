//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2516/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2516<F: Float>(t45384: F, t487: F, t1269: F, t3552: F, t44420: F, t12690: F, t44831: F, t12657: F, t1204: F, t3727: F, t3555: F, t13180: F, t493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45449 = t45384 * t487;
    let t45464 = t3552 * t1269;
    let t45482 = t44420 * t487;
    let t45487 = t12690 * t487;
    let t45515 = t44831 * t487;
    let t45522 = t12657 * t1269;
    let t45535 = t1204 * t3727;
    let t45545 = t3555 * t3727;
    let t45551 = F::cast_from(1.0_f64) / t13180 / t493;
    (t45449, t45464, t45482, t45487, t45515, t45522, t45535, t45545, t45551)
}
