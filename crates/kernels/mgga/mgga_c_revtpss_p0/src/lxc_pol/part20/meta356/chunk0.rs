//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1298/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1298<F: Float>(t2452: F, t588: F, t258: F, t2454: F, t2455: F, t39494: F, t10985: F, t11018: F, t2439: F, t2760: F, t780: F, t785: F) -> (F, F, F, F, F) {
    let t39552 = t588 * t2452;
    let t39554 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t258;
    let t39557 = F::cast_from(0.20561456923286030469e-1_f64) * t2454 * t2455 * t39494;
    let t39558 = t11018 * t10985;
    let t39562 = t2439 * t785 * t2760 * t780;
    (t39552, t39554, t39557, t39558, t39562)
}
