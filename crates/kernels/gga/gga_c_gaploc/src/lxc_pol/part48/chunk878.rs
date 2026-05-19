//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 878/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk878<F: Float>(t13507: F, t7129: F, t11603: F, t2530: F, t2508: F, t7226: F, t13556: F, t7137: F, t13525: F, t795: F, t11595: F, t1897: F, t7671: F) -> (F, F, F, F, F, F) {
    let t44994 = F::cast_from(0.46143157380853345701e-1_f64) * t7129 * t13507;
    let t44995 = t11603 * t2530;
    let t44998 = F::cast_from(0.46143157380853345701e-1_f64) * t2508 * t7226 * t44995;
    let t45000 = F::cast_from(0.20508069947045931423e-1_f64) * t7137 * t13556;
    let t45001 = t795 * t13525;
    let t45009 = F::cast_from(0.23071578690426672851e-1_f64) * t1897 * t11595 * t7671;
    (t44994, t44995, t44998, t45000, t45001, t45009)
}
