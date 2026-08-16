//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 947/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk947(t32125: f64, t5608: f64, t22602: f64, t7837: f64, t1614: f64, t58: f64, t22849: f64, t7178: f64, t15: f64, t32139: f64) -> (f64, f64, f64, f64, f64) {
    let t136812 = t32125 * t5608;
    let t136814 = t7837 * t22602;
    let t136815 = t1614 * t58;
    let t136822 = t7178 * t22849;
    let t136825 = t32139 * t15;
    (t136812, t136814, t136815, t136822, t136825)
}
