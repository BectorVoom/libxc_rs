//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1054/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1054<F: Float>(t322: F, t1035: F, t2983: F, t2987: F, t352: F, t6755: F, t10468: F, t6767: F, t10484: F, t10486: F, t10489: F, t10528: F, t1348: F, t2437: F, t330: F, t3675: F, t855: F, t9773: F) -> (F, F, F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t332 = 0.25e1 < t322;
    let t10529 = t2983 * t1035;
    let t10533 = t352 * t2987;
    let t10536 = t6755 * t10529;
    let t10539 = piecewise3(t332, t10468, 0.0);
    let t10545 = t6767 * t10529;
    let t10549 = piecewise5(t323, t10484 * t330 + 3.0 * t10486 * t330 + t10489 * t330, t331, t10528, -0.63e1 * t1348 * t10529 * t352 - 0.63e1 * t2437 * t10533 - 0.945e1 * t10536 * t352 - 0.105e1 * t855 * t10539 * t352 - 0.4725e1 * t9773 * t3675 - 0.23625e1 * t10545 * t352);
    (t10529, t10533, t10536, t10539, t10545, t10549)
}
