//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1306/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1306<F: Float>(t27651: F, t8209: F, t27556: F, t28772: F, t94621: F, t94624: F, t95130: F, t98663: F, t98666: F, t98673: F, t98676: F, t98680: F, t98684: F) -> F {
    let t99524 = t8209 * t27651;
    let t99534 = F::new(0.7722800925925925926e-4) * t95130 + F::new(0.46429444444444444443e-2) * t98663 - F::new(0.15476481481481481481e-2) * t98666 + F::new(0.7722800925925925926e-4) * t99524 - F::new(0.17411041666666666666e-2) * t98673 + F::new(0.61905925925925925924e-2) * t98676 + F::new(0.23214722222222222222e-2) * t98680 + F::new(0.51588271604938271604e-3) * t98684 + F::new(0.92754700520833333334e-4) * t27556 * t28772 - F::new(0.25794135802469135802e-3) * t94621 - F::new(0.23214722222222222222e-2) * t94624;
    t99534
}
