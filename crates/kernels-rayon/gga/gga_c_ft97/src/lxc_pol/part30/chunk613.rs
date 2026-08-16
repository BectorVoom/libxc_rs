//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 613/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk613(t27761: f64, t27794: f64, t27840: f64, t27887: f64, t762: f64, t17712: f64, t2: f64, t4: f64, t26: f64, t3972: f64, t6154: f64, t13830: f64, t1449: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27889 = t27761 + t27794 + t27840 + t27887;
    let t27890 = t762 * t27889;
    let t27892 = t17712 * t2;
    let t27893 = t27892 * t4;
    let t27894 = t27893 * t26;
    let t27897 = t6154 * t3972;
    let t27899 = t13830 * t1449;
    (t27889, t27890, t27892, t27894, t27897, t27899)
}
