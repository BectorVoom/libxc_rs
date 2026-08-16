//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1278/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1278<F: Float>(t2615: F, t326: F, t33627: F, t10914: F, t10915: F, t32893: F, t28818: F, t3005: F, t7419: F, t9800: F, t1890: F, t32356: F) -> (F, F, F, F, F) {
    let t33716 = F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t33627;
    let t33722 = F::cast_from(0.21450293971110256001e1_f64) * t10914 * t10915 * t32893;
    let t33728 = F::cast_from(0.63904876589867916128e-1_f64) * t28818;
    let t33731 = t9800 * t3005 * t7419;
    let t33732 = F::cast_from(0.36425779656224712192e1_f64) * t33731;
    let t33760 = t1890 * t32356;
    (t33716, t33722, t33728, t33732, t33760)
}
