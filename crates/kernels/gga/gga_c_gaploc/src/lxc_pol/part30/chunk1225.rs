//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1225/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1225<F: Float>(t10677: F, t1880: F, t21636: F, t3440: F, t3420: F, t21556: F, t2554: F, t7064: F, t8871: F, t1897: F, t7671: F, t8637: F) -> (F, F, F, F, F, F) {
    let t32387 = t10677 * t1880;
    let t32394 = F::cast_from(0.10254034973522965712e-1_f64) * t21636 * t3440;
    let t32398 = F::cast_from(0.34180116578409885707e-2_f64) * t21636 * t3420;
    let t32400 = F::cast_from(0.6152420984113779427e-1_f64) * t21556 * t3440;
    let t32407 = t7064 * t8871 * t2554;
    let t32408 = F::cast_from(0.64087718584518535698e-3_f64) * t32407;
    let t32411 = F::cast_from(0.46143157380853345702e-1_f64) * t1897 * t8637 * t7671;
    (t32387, t32394, t32398, t32400, t32408, t32411)
}
