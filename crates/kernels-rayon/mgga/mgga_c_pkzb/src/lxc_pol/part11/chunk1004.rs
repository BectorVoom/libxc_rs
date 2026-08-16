//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1004/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1004(t11043: f64, t2105: f64, t11011: f64, t11015: f64, t11021: f64, t11025: f64, t11030: f64, t11034: f64, t11039: f64, t2104: f64, t276: f64, t2899: f64, t2922: f64, t9547: f64, t9584: f64, t9606: f64, t9614: f64, t9617: f64, t9623: f64, t9629: f64) -> (f64, f64) {
    let t11044 = t2105 * t11043;
    let t11052 = -t9547 / 96.0_f64 - t276 * t11011 / 96.0_f64 - t276 * t11015 / 16.0_f64 + t9584 / 48.0_f64 + 0.38586616306262763275e-2_f64 * t2104 * t11021 + 0.12862205435420921092e-2_f64 * t2922 * t11025 - 0.64311027177104605458e-3_f64 * t2922 * t11030 - 0.12862205435420921092e-2_f64 * t2104 * t11034 - 0.12862205435420921092e-2_f64 * t2104 * t11039 - 0.25724410870841842183e-2_f64 * t2899 * t11044 - 0.42874018118069736972e-3_f64 * t9606 + 0.85748036236139473944e-3_f64 * t9614 + 0.42874018118069736972e-3_f64 * t9617 - 0.85748036236139473944e-3_f64 * t9623 + 0.25724410870841842184e-2_f64 * t9629;
    (t11044, t11052)
}
