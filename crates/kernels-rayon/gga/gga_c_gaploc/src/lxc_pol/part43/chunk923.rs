//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 923/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk923(t43756: f64, t13052: f64, t1966: f64, t28673: f64, t2615: f64, t326: f64, t43683: f64, t8775: f64, t9842: f64, t41231: f64, t41237: f64, t41244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43757 = 0.19171462976960374838e1_f64 * t43756;
    let t43758 = t1966 * t13052;
    let t43759 = 0.25561950635947166451e1_f64 * t43758;
    let t43760 = t28673 * t13052;
    let t43761 = 0.19171462976960374838e1_f64 * t43760;
    let t43766 = 0.46011511144704899612e1_f64 * t2615 * t326 * t43683;
    let t43774 = 0.11916829983950142223e0_f64 * t8775 * t9842;
    let t43775 = 0.63904876589867916127e-1_f64 * t41231;
    let t43777 = 0.29792074959875355558e-1_f64 * t41237;
    let t43778 = 0.63904876589867916127e-1_f64 * t41244;
    (t43757, t43759, t43761, t43766, t43774, t43775, t43777, t43778)
}
