//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 838/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk838<F: Float>(t1613: F, t5555: F, t409: F, t5517: F, t1301: F, t136505: F, t32259: F, t32125: F, t5608: F, t22602: F, t7837: F, t1614: F, t58: F, t22849: F, t7178: F, t15: F, t32139: F) -> (F, F, F, F, F, F, F, F) {
    let t136759 = t1613 * t5555;
    let t136772 = t5517 * t409;
    let t136807 = t32259 * t1301 * t136505;
    let t136812 = t32125 * t5608;
    let t136814 = t7837 * t22602;
    let t136815 = t1614 * t58;
    let t136822 = t7178 * t22849;
    let t136825 = t32139 * t15;
    (t136759, t136772, t136807, t136812, t136814, t136815, t136822, t136825)
}
