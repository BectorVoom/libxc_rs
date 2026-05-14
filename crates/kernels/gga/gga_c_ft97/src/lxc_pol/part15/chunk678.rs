//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 678/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk678<F: Float>(t20781: F, t20850: F, t143: F, t160: F, t20224: F, t3440: F, t3439: F, t13153: F, t4823: F, t12680: F, t4828: F, t17021: F, t925: F, t9133: F, t12752: F, t16986: F, t17060: F, t17091: F, t1901: F, t20744: F, t20750: F, t20755: F, t20760: F, t20765: F, t20769: F, t28: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20851 = t20781 + t20850;
    let t20853 = t143 * t20851 * t160;
    let t20858 = t3440 * t20224;
    let t20859 = t3439 * t20858;
    let t20862 = t13153 * t4823;
    let t20865 = t12680 * t4828;
    let t20868 = t17021 * t925;
    let t20869 = t9133 * t20868;
    let t20872 = -2.0 / 9.0 * t16986 - 2.0 / 3.0 * t1901 * t20744 + t17060 / 3.0 + 2.0 / 9.0 * t1901 * t20750 + 2.0 / 9.0 * t1901 * t20755 + t1901 * t20760 / 3.0 + t1901 * t20765 / 3.0 + 2.0 / 3.0 * t1901 * t20769 - 2.0 / 3.0 * t17091 + t89 * t28 * t20853 / 3.0 + 4.0 / 9.0 * t12752 - 2.0 / 9.0 * t1901 * t20859 + 2.0 / 3.0 * t1901 * t20862 + 2.0 / 3.0 * t1901 * t20865 - 2.0 / 3.0 * t1901 * t20869;
    (t20851, t20853, t20858, t20859, t20862, t20865, t20868, t20869, t20872)
}
