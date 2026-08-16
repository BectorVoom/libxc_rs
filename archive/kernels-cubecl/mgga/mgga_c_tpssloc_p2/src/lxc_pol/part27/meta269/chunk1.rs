//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1293/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1293<F: Float>(t265: F, t394: F, t1484: F, t1915: F, t202: F, t7540: F, t1530: F, t1877: F, t193: F, t2522: F, t6670: F, t870: F, t1070: F, t1637: F, t336: F, t4700: F, t6822: F, t7627: F) -> (F, F, F) {
    let t395 = t265 < t394;
    let t7634 = t1915 * t1484;
    let t7637 = t202 * t7540;
    let t7642 = -t1530 * t1877 * t6670 + t193 * t7637 * t870 + F::cast_from(3.0_f64) * t2522 * t7634;
    let t7643 = piecewise3::<F>(t395, t1070 * t193 * t336 * t7627 - t1637 * t4700 * t6822, t7642);
    (t7634, t7642, t7643)
}
