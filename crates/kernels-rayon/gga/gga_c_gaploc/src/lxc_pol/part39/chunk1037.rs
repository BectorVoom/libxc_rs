//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1037/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1037(t43756: f64, t13052: f64, t1966: f64, t28673: f64, t2676: f64, t33139: f64, t2615: f64, t326: f64, t43683: f64, t43490: f64, t6066: f64, t6111: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43757 = 0.19171462976960374838e1_f64 * t43756;
    let t43758 = t1966 * t13052;
    let t43759 = 0.25561950635947166451e1_f64 * t43758;
    let t43760 = t28673 * t13052;
    let t43761 = 0.19171462976960374838e1_f64 * t43760;
    let t43762 = t33139 * t2676;
    let t43766 = 0.46011511144704899612e1_f64 * t2615 * t326 * t43683;
    let t43768 = t6111 * t6066 * t43490;
    (t43757, t43759, t43761, t43762, t43766, t43768)
}
