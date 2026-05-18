//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1209/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1209<F: Float>(t3597: F, t7566: F, t11837: F, t2651: F, t10698: F, t12526: F, t10894: F, t3063: F, t261: F, t7628: F, t8865: F, t7614: F, t8872: F) -> (F, F, F, F, F, F) {
    let t43424 = t7566 * t3597;
    let t43426 = t2651 * t11837;
    let t43428 = t10698 * t12526;
    let t43432 = t10894 * t3063;
    let t43435 = t7628 * t261 * t8865;
    let t43438 = t7614 * t261 * t8872;
    (t43424, t43426, t43428, t43432, t43435, t43438)
}
