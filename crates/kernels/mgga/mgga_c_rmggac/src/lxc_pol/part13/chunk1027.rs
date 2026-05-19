//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1027/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1027<F: Float>(t9371: F, t8623: F, t8627: F, t8633: F, t8637: F, t8643: F, t8647: F, t42468: F, t42469: F, t42470: F, t42471: F, t8651: F) -> (F, F) {
    let t42472 = F::cast_from(0.11974241701863808564e0_f64) * t9371;
    let t42473 = F::cast_from(0.2727466165424534173e-1_f64) * t8623;
    let t42474 = F::cast_from(0.16364796992547205038e0_f64) * t8627;
    let t42475 = F::cast_from(0.2727466165424534173e0_f64) * t8633;
    let t42476 = F::cast_from(0.5454932330849068346e-1_f64) * t8637;
    let t42477 = F::cast_from(0.81823984962736025192e-1_f64) * t8643;
    let t42478 = F::cast_from(0.16364796992547205038e0_f64) * t8647;
    let t42479 = t42468 + t42469 - t42470 - t42471 + t42472 + t42473 - t42474 + t42475 + t42476 + t42477 - t42478;
    let t42484 = F::cast_from(0.40911992481368012596e-1_f64) * t8651;
    (t42479, t42484)
}
