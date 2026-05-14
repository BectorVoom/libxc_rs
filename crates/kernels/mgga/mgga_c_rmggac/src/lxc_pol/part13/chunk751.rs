//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 751/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk751<F: Float>(t7345: F, t8349: F, t1665: F, t2010: F, t7359: F, t1661: F, t7335: F, t2415: F, t4025: F, t2011: F, t291: F, t5354: F, t7508: F, t8533: F, t194: F, t1979: F, t1982: F, t201: F, t5530: F) -> (F, F, F, F, F, F, F, F) {
    let t38749 = t7345 * t8349;
    let t38752 = t2010 * t7359 * t1665;
    let t38755 = t2010 * t7359 * t1661;
    let t38757 = t7335 * t8349;
    let t38760 = t2010 * t2415 * t4025;
    let t38764 = t2010 * t2011 * t5354 * t291;
    let t38775 = t7508 * t8533;
    let t38780 = t194 * t5530 * t201 * t1979 * t1982;
    (t38749, t38752, t38755, t38757, t38760, t38764, t38775, t38780)
}
