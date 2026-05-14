//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 962/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk962<F: Float>(t25162: F, t35843: F, t143038: F, t143058: F, t152659: F, t152663: F, t152667: F, t152671: F, t152675: F, t152680: F, t152686: F, t152690: F, t152694: F, t152698: F, t152702: F, t152704: F, t152708: F) -> (F, F) {
    let t152710 = t25162 * t35843;
    let t152712 = -t143038 / 54.0 - t152659 / 3.0 - t152663 / 3.0 - t152667 / 36.0 - 2.0 / 9.0 * t152671 - 4.0 / 9.0 * t152675 + 2.0 / 27.0 * t152680 + t143058 / 27.0 - 5.0 / 4.0 * t152686 - t152690 / 6.0 - t152694 / 8.0 + 8.0 / 3.0 * t152698 + t152702 + 4.0 / 9.0 * t152704 - 2.0 * t152708 + t152710 / 27.0;
    (t152710, t152712)
}
