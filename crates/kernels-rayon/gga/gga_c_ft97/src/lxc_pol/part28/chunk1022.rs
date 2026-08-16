//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1022/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1022(t32094: f64, t3266: f64, t38921: f64, t5674: f64, t25996: f64, t5675: f64, t8411: f64, t25878: f64, t3052: f64, t7824: f64, t22952: f64, t25883: f64) -> (f64, f64, f64, f64) {
    let t144866 = t5674 * t38921 * t32094 * t3266;
    let t144870 = t5674 * t8411 * t5675 * t25996;
    let t144874 = t25878 * t7824 * t32094 * t3052;
    let t144878 = t22952 * t8411 * t32094 * t25883;
    (t144866, t144870, t144874, t144878)
}
