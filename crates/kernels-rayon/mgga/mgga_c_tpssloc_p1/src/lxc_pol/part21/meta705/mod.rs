//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2537;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta705(t3067: f64, t353: f64, t373: f64, t383: f64, t1021: f64, t820: f64, t10482: f64, t1615: f64, t1041: f64, t13969: f64, t14142: f64, t14179: f64, t10375: f64, t1612: f64, t1539: f64, t248: f64, t42749: f64, t10661: f64, t1556: f64, t14363: f64, t300: f64, t14419: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48607, t48611, t48612, t48626, t48629) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2537(t3067, t353, t373, t383, t1021, t820, t10482, t1615, t1041, t13969, t14142, t14179);
        let (t48670, t48674, t48763, t48766, t48771) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2538(t10375, t1612, t1041, t1539, t248, t42749, t10661, t1556, t14363, t300, t14419, t923);
    (t48607, t48611, t48612, t48626, t48629, t48670, t48674, t48763, t48766, t48771)
}
