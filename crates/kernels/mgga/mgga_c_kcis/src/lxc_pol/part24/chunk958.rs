//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 958/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk958<F: Float>(t26685: F, t26692: F, t27766: F, t27770: F, t27775: F, t27780: F, t27786: F, t27790: F, t27794: F, t27797: F, t27799: F, t27804: F, t27808: F, t27812: F, t27816: F, t27822: F, t27826: F, t27832: F, t7703: F, t7706: F, t8038: F) -> (F,) {
    let t27835 = 0.49745833333333333332e-2 * t27766 - 0.16581944444444444444e-2 * t27770 - 0.13901041666666666667e-2 * t7703 * t27775 - 0.69505208333333333333e-3 * t7703 * t27780 - 0.92754700520833333333e-4 * t26685 * t27780 + 0.11054629629629629629e-2 * t27786 - 0.16581944444444444444e-2 * t27790 - 0.55273148148148148147e-3 * t27794 - 0.44218518518518518517e-2 * t27797 + 0.11054629629629629629e-2 * t27799 + 0.61782407407407407408e-3 * t26692 * t8038 - 0.7722800925925925926e-4 * t27804 - 0.2782641015625e-3 * t26685 * t27808 - 0.185671721767578125e-4 * t27812 * t27808 + 0.23168402777777777778e-3 * t7703 * t27816 + 0.23168402777777777778e-3 * t7703 * t27822 + 0.46336805555555555556e-3 * t7703 * t27826 + 0.30918233506944444445e-4 * t26685 * t27822 - 0.23168402777777777778e-3 * t27832 * t7706;
    (t27835,)
}
