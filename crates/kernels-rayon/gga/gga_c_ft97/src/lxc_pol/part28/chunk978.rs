//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 978/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk978(t173: f64, t32858: f64, t7195: f64, t23825: f64, t136840: f64, t32764: f64, t32318: f64, t94760: f64, t23721: f64, t2058: f64, t5555: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t138838 = t7195 * t173 * t32858;
    let t138839 = t23825 * t138838;
    let t138843 = t32764 * t136840;
    let t138854 = t94760 * t32318;
    let t138857 = t23721 * t32318;
    let t138866 = t2058 * t5555;
    let t138867 = t542 * t138866;
    (t138838, t138839, t138843, t138854, t138857, t138866, t138867)
}
