//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 629/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk629<F: Float>(t1707: F, t8708: F, t4881: F, t8701: F, t1714: F, t1248: F, t4893: F, t8510: F, t1720: F, t8514: F, t8518: F, t4876: F, t4888: F, t7076: F, t7122: F, t8684: F, t8687: F, t8690: F, t8702: F) -> (F, F, F, F, F, F, F) {
    let t8709 = t1707 * t8708;
    let t8715 = t4881 * t8701;
    let t8717 = t1714 * t8708;
    let t8721 = t1248 * t4893 * t8510;
    let t8724 = t1248 * t1720 * t8514;
    let t8727 = t1248 * t1720 * t8518;
    let t8729 = -F::cast_from(0.9494625e0_f64) * t8702 + F::cast_from(0.1898925e1_f64) * t8709 + t4876 + F::cast_from(0.19931111111111111111e0_f64) * t7076 - F::cast_from(0.19931111111111111111e0_f64) * t8684 + F::cast_from(0.59793333333333333334e0_f64) * t8687 - F::cast_from(0.29896666666666666667e0_f64) * t8690 + F::cast_from(0.15358125e0_f64) * t8715 + F::cast_from(0.3071625e0_f64) * t8717 + t4888 + F::cast_from(0.21908444444444444444e0_f64) * t7122 - F::cast_from(0.5477111111111111111e-1_f64) * t8721 + F::cast_from(0.32862666666666666666e0_f64) * t8724 - F::cast_from(0.16431333333333333333e0_f64) * t8727;
    (t8709, t8715, t8717, t8721, t8724, t8727, t8729)
}
