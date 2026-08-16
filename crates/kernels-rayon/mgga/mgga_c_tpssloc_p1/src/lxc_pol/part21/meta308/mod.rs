//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1656;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1657;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta308(t11570: f64, t2244: f64, t3448: f64, t3469: f64, t2250: f64, t3450: f64, t3247: f64, t460: f64, t1176: f64, t134: f64, t1184: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11571, t11575, t11579, t11583) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1656(t11570, t2244, t3448, t3469, t2250, t3450, t3247, t460);
        let (t11584, t11588) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1657(t11583, t2244, t1176, t134);
        let t11589 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1658(t11588, t1184);
    (t11571, t11575, t11579, t11583, t11584, t11588, t11589)
}
