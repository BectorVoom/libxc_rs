//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 770/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk770<F: Float>(t8585: F, t8588: F, t8590: F, t8593: F, t8595: F, t8598: F, t8604: F, t8610: F, t8612: F, t8617: F, t9371: F, t8623: F, t8627: F, t8633: F, t8637: F, t8643: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42461 = 0.5107751987195740728e-4 * t8585;
    let t42462 = 0.5107751987195740728e-4 * t8588;
    let t42463 = 0.5107751987195740728e-4 * t8590;
    let t42464 = 0.5107751987195740728e-4 * t8593;
    let t42465 = 0.1702583995731913576e-4 * t8595;
    let t42466 = 0.1702583995731913576e-4 * t8598;
    let t42468 = 0.1702583995731913576e-4 * t8604;
    let t42469 = 0.1702583995731913576e-4 * t8610;
    let t42470 = 0.212822999466489197e-4 * t8612;
    let t42471 = 0.212822999466489197e-4 * t8617;
    let t42472 = 0.11974241701863808564e0 * t9371;
    let t42473 = 0.2727466165424534173e-1 * t8623;
    let t42474 = 0.16364796992547205038e0 * t8627;
    let t42475 = 0.2727466165424534173e0 * t8633;
    let t42476 = 0.5454932330849068346e-1 * t8637;
    let t42477 = 0.81823984962736025192e-1 * t8643;
    (t42461, t42462, t42463, t42464, t42465, t42466, t42468, t42469, t42470, t42471, t42472, t42473, t42474, t42475, t42476, t42477)
}
