//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1282/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1282(t2615: f64, t326: f64, t33627: f64, t10914: f64, t10915: f64, t32893: f64, t28818: f64, t3005: f64, t7419: f64, t9800: f64, t1890: f64, t32356: f64) -> (f64, f64, f64, f64, f64) {
    let t33716 = 0.92023022289409799224e1_f64 * t2615 * t326 * t33627;
    let t33722 = 0.21450293971110256001e1_f64 * t10914 * t10915 * t32893;
    let t33728 = 0.63904876589867916128e-1_f64 * t28818;
    let t33731 = t9800 * t3005 * t7419;
    let t33732 = 0.36425779656224712192e1_f64 * t33731;
    let t33760 = t1890 * t32356;
    (t33716, t33722, t33728, t33732, t33760)
}
