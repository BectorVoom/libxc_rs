//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1890;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta543(t52: f64, t8027: f64, t2136: f64, t461: f64, t7573: f64, t7324: f64, t3448: f64, t4729: f64, t475: f64, t5011: f64, t68: f64, t7328: f64, t4724: f64, t4899: f64, t1210: f64, t8039: f64, t24721: f64, t1714: f64, t2133: f64, t2132: f64, t6739: f64, t8026: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27681, t27683, t27684, t27687, t27691, t27692) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1890(t52, t8027, t2136, t461, t7573, t7324, t3448, t4729, t475, t5011, t68, t7328);
        let (t27697, t27700, t27701, t27703, t27704, t27710) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1891(t4724, t4899, t1210, t8039, t24721, t1714, t2133, t2132, t6739, t8026);
    (t27681, t27683, t27684, t27687, t27691, t27692, t27697, t27700, t27701, t27703, t27704, t27710)
}
