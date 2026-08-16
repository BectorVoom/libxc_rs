//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2055;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta555(t2559: f64, t2570: f64, t782: f64, t9558: f64, t2617: f64, t9600: f64, t786: f64, t9569: f64, t805: f64, t222: f64, t39934: f64, t9637: f64, t2691: f64, t812: f64, t815: f64, t10024: f64, t809: f64, t238: f64, t244: f64, t248: f64, t40445: f64, t9525: f64, t9577: f64, t116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41008, t41011, t41052, t41083, t41084, t41096, t41107) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2055(t2559, t2570, t782, t9558, t2617, t9600, t786, t9569, t805, t222, t39934, t9637);
        let (t41115, t41130, t41139, t41144, t41146) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2056(t2691, t812, t815, t10024, t809, t238, t244, t248, t40445, t9525, t9577, t116);
    (t41008, t41011, t41052, t41083, t41084, t41096, t41107, t41115, t41130, t41139, t41144, t41146)
}
