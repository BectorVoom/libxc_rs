//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 757/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk757<F: Float>(t2010: F, t2415: F, t4025: F, t2011: F, t291: F, t5354: F, t7508: F, t8533: F, t194: F, t1979: F, t1982: F, t201: F, t5530: F, t2134: F, t27: F, t3118: F, t551: F) -> (F, F, F, F, F) {
    let t38760 = t2010 * t2415 * t4025;
    let t38764 = t2010 * t2011 * t5354 * t291;
    let t38775 = t7508 * t8533;
    let t38776 = 0.18183107769496894486e-1 * t38775;
    let t38780 = t194 * t5530 * t201 * t1979 * t1982;
    let t38784 = t2134 * t27 * t3118 * t551;
    (t38760, t38764, t38776, t38780, t38784)
}
