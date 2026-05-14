//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 931/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk931<F: Float>(t1286: F, t34353: F, t376: F, t1852: F, t34511: F, t492: F, t144769: F, t144773: F, t144777: F, t144781: F, t144786: F, t144790: F, t144794: F, t144798: F, t144803: F, t144805: F, t144807: F, t144811: F, t144815: F, t144817: F, t144820: F, t144824: F) -> (F, F, F) {
    let t145771 = t1286 * t376 * t34353;
    let t145774 = t1852 * t34511 * t492;
    let t145790 = -t144769 / 6.0 - t144773 + t144777 + 8.0 / 3.0 * t144781 - 5.0 / 4.0 * t144786 + t144790 / 9.0 - 4.0 / 9.0 * t144794 + t144798 / 18.0 + 4.0 / 9.0 * t144803 - 4.0 / 27.0 * t144805 - t144807 / 54.0 - t144811 / 9.0 + t144815 / 3.0 - 2.0 / 9.0 * t144817 + 2.0 / 9.0 * t144820 + t144824 / 27.0;
    (t145771, t145774, t145790)
}
