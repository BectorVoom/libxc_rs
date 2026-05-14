//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 713/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk713<F: Float>(t32729: F, t609: F, t1384: F, t23478: F, t2142: F, t7407: F, t23408: F, t5778: F, t28: F, t1349: F, t1362: F, t32686: F, t32692: F, t32696: F, t32701: F, t32703: F, t32708: F, t32711: F, t32714: F, t32719: F, t32724: F, t32727: F, t564: F, t5766: F, t5772: F, t5775: F, t5845: F, t7309: F, t7346: F, t7412: F) -> (F, F, F, F, F, F) {
    let t32730 = t32729 * t609;
    let t32732 = t23478 * t1384;
    let t32735 = t2142 * t7407;
    let t32737 = t5778 * t23408;
    let t32738 = t28 * t32737;
    let t32741 = t32686 * t1362 / 6.0 + t5766 * t7346 / 3.0 + t1349 * t32692 / 3.0 + t1349 * t32696 / 3.0 - t32701 - t32703 + t7309 * t5845 / 6.0 - t32708 - t1349 * t32711 / 3.0 - t32714 * t5775 / 18.0 + t5772 * t32719 / 9.0 - t5772 * t32724 / 18.0 - 4.0 * t32727 - 2.0 * t32730 - 4.0 * t32732 - t564 * t7412 - 2.0 * t32735 - 2.0 / 3.0 * t1349 * t32738;
    (t32730, t32732, t32735, t32737, t32738, t32741)
}
