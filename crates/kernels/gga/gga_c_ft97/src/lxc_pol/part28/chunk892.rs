//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 892/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk892<F: Float>(t144801: F, t22958: F, t5674: F, t144792: F, t137082: F, t3188: F, t25928: F, t144769: F, t144773: F, t144777: F, t144781: F, t144786: F, t144790: F, t144794: F, t144798: F, t144803: F, t144805: F, t144807: F, t144811: F, t144815: F) -> (F, F, F, F, F) {
    let t144817 = t5674 * t22958 * t144801;
    let t144820 = t5674 * t22958 * t144792;
    let t144822 = t137082 * t3188;
    let t144824 = t5674 * t25928 * t144822;
    let t144826 = -t144769 / 2.0 - 3.0 * t144773 + 3.0 * t144777 + 8.0 * t144781 - 15.0 / 4.0 * t144786 + t144790 / 3.0 - 4.0 / 3.0 * t144794 + t144798 / 6.0 + 4.0 / 3.0 * t144803 - 4.0 / 9.0 * t144805 - t144807 / 18.0 - t144811 / 3.0 + t144815 - 2.0 / 3.0 * t144817 + 2.0 / 3.0 * t144820 + t144824 / 9.0;
    (t144817, t144820, t144822, t144824, t144826)
}
