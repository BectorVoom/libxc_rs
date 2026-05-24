//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 465/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk465<F: Float>(t1192: F, t3634: F, t1170: F, t317: F, t305: F, t1190: F, t1191: F, t3571: F, t303: F, t3559: F, t1180: F, t3587: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3636 = F::new(2.0) * t3634 * t1192;
    let t3637 = t1170 * t317;
    let t3638 = F::new(1.0) / t3637;
    let t3639 = t305 * t3638;
    let t3640 = t1190 * t1190;
    let t3641 = t3640 * t1191;
    let t3643 = F::new(2.0) * t3639 * t3641;
    let t3646 = F::cast_from(0.39862222222222222223e0_f64) * t3571;
    let t3651 = F::new(1.0)/F::sqrt(t303);
    let t3652 = t3651 * t3559;
    let t3654 = t1180 * t3587;
    (t3636, t3638, t3639, t3640, t3641, t3643, t3646, t3651, t3652, t3654)
}
