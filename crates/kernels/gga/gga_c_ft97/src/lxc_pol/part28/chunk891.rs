//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 891/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk891<F: Float>(t5617: F, t5691: F, t920: F, t136151: F, t32067: F, t136240: F, t34376: F, t23054: F, t34412: F, t34415: F, t379: F, t22958: F, t5674: F, t3204: F, t32333: F, t93355: F) -> (F, F, F, F, F, F, F, F) {
    let t144801 = t5691 * t920 * t5617;
    let t144803 = t32067 * t136151 * t144801;
    let t144805 = t136240 * t34376;
    let t144807 = t23054 * t34412;
    let t144809 = t34415 * t379;
    let t144811 = t5674 * t22958 * t144809;
    let t144813 = t32333 * t3204;
    let t144815 = t5674 * t93355 * t144813;
    (t144801, t144803, t144805, t144807, t144809, t144811, t144813, t144815)
}
