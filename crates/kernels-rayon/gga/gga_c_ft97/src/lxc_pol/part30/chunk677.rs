//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 677/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk677(t1486: f64, t681: f64, t7071: f64, t10248: f64, t28760: f64, t446: f64, t1234: f64, t6260: f64, t852: f64, t193: f64, t6308: f64, t1476: f64, t4226: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28811 = t1486 * t681 * t7071;
    let t28813 = t10248 * t28760;
    let t28814 = t446 * t28813;
    let t28816 = t6260 * t1234;
    let t28817 = t852 * t28816;
    let t28819 = t6308 * t193 * t28817;
    let t28821 = t1476 * t4226;
    (t28811, t28813, t28814, t28816, t28819, t28821)
}
