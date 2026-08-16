//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2286/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2286<F: Float>(t17624: F, t6717: F, t1933: F, t1937: F, t5398: F, t1022: F, t10475: F, t17738: F, t23422: F, t23678: F, t25609: F, t25652: F, t25653: F, t25654: F, t28578: F, t3128: F, t4649: F, t5866: F, t5872: F, t5885: F, t7574: F, t7583: F, t82516: F, t82542: F, t82911: F, t88286: F, t88415: F, t88537: F) -> F {
    let t99624 = t6717 * t17624;
    let t99631 = t1933 * t5398 * t1937;
    let t99635 = F::cast_from(0.20186378047070195428e-3_f64) * t25652 * t3128 * t5866 * t25654 + F::cast_from(0.40372756094140390856e-3_f64) * t25652 * t25653 * t23678 * t4649 - F::cast_from(0.20186378047070195428e-3_f64) * t82911 * t28578 + F::cast_from(0.60559134141210586284e-3_f64) * t88537 * t10475 * t5872 * t82516 * t1022 - F::cast_from(0.60559134141210586284e-3_f64) * t88537 * t3128 * t5872 * t82542 * t1022 + t23422 * t5885 / F::cast_from(54.0_f64) - t99624 / F::cast_from(432.0_f64) - t88415 - F::cast_from(0.16149102437656156342e-2_f64) * t88286 * t7583 - F::cast_from(0.20186378047070195428e-3_f64) * t7574 * t25609 + F::cast_from(0.10093189023535097714e-3_f64) * t99631 + t6717 * t17738 / F::cast_from(288.0_f64);
    t99635
}
