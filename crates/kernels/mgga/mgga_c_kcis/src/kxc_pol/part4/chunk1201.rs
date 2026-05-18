//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1201/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1201<F: Float>(t13712: F, t10923: F, t10924: F, t13710: F, t13714: F, t13723: F, t13732: F, t13767: F, t13942: F, t13945: F, t13949: F, t13717: F, t13742: F, t13772: F, t13775: F, t13777: F, t13881: F, t13886: F, t13888: F, t13892: F, t13912: F, t13915: F, t13918: F, t13921: F, t13924: F, t13927: F, t13931: F, t13934: F, t15398: F, t9681: F, t9683: F, t9691: F) -> F {
    let t15411 = F::new(0.22954444444444444444e0) * t13712;
    let t15420 = t15411 - F::new(0.68863333333333333333e0) * t13714 + F::new(0.20659e1) * t13723 - F::new(0.309885e1) * t13732 - t10923 - t10924 + F::new(0.6311625e0) * t13942 + F::new(0.3529725e1) * t13767 - F::new(0.11577222222222222222e0) * t13945 - F::new(0.22954444444444444444e0) * t13710 + F::new(0.90302333333333333334e0) * t13949;
    let t15422 = F::new(0.264729375e1) * t13772 - F::new(0.157790625e0) * t13881 - F::new(0.3529725e1) * t13775 - F::new(0.17648625e1) * t13777 + F::new(0.6311625e0) * t13886 + F::new(0.31558125e0) * t13888 - F::new(0.20839e0) * t13892 + F::new(0.17215833333333333333e0) * t9681 + F::new(0.11477222222222222222e0) * t9683 - F::new(0.45908888888888888888e0) * t9691 + t15398 + F::new(0.46308888888888888889e-1) * t13912 - F::new(0.34731666666666666667e-1) * t13915 - F::new(0.46308888888888888889e-1) * t13918 - F::new(0.13892666666666666667e0) * t13921 + F::new(0.20839e0) * t13924 + F::new(0.83356e0) * t13927 + F::new(0.37874833333333333334e1) * t13717 + F::new(0.20839e0) * t13931 - F::new(0.62517e0) * t13934 - F::new(0.103295e1) * t13742 + t15420;
    t15422
}
