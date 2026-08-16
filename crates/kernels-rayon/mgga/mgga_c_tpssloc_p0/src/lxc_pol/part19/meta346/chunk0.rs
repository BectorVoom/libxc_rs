//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1246/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1246(t2784: f64, t2841: f64, t2845: f64, t10697: f64, t2787: f64, t10696: f64, t2842: f64, t2844: f64, t912: f64, t10702: f64, t10704: f64, t2793: f64, t2836: f64) -> (f64, f64, f64, f64) {
    let t41623 = t2784 * t2841;
    let t41625 = 0.96491876992155210402e2_f64 * t41623 * t2845;
    let t41627 = 4.0_f64 * t2787 * t10697;
    let t41635 = 0.64327917994770140268e2_f64 * t2842 * t10696 * t2844 * t912;
    let t41639 = 0.3103560775156404018e4_f64 * t10702 * t2793 * t10704 * t2836;
    (t41625, t41627, t41635, t41639)
}
