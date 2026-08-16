//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 844/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk844(t12663: f64, t12707: f64, t12774: f64, t12996: f64, t13037: f64, t13081: f64, t13184: f64, t13225: f64, t12939: f64, t160: f64, t1022: f64, t8787: f64) -> (f64, f64, f64) {
    let t13228 = t12663 + t12707 + t12774 + t12996 + t13037 + t13081 + t13184 + t13225;
    let t13230 = t12939 * t160;
    let t13234 = t8787 * t1022;
    (t13228, t13230, t13234)
}
