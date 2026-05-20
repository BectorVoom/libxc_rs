//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3485/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3485<F: Float>(t11656: F, t11994: F, t15707: F, t16140: F, t1671: F, t19895: F, t43268: F, t4825: F, t53359: F, t53363: F, t53692: F, t54739: F, t6308: F, t65567: F, t65570: F, t65581: F, t65585: F, t65589: F) -> F {
    let t65591 = F::cast_from(0.3811023832717309953e-3_f64) * t53359 + F::cast_from(0.19055119163586549765e-3_f64) * t53363 + F::cast_from(0.31758531939310916276e-3_f64) * t65567 - F::cast_from(0.3811023832717309953e-3_f64) * t65570 + F::cast_from(0.57165357490759649296e-3_f64) * t11994 * t19895 - F::cast_from(0.30488190661738479624e-2_f64) * t11656 * t19895 - F::cast_from(0.57165357490759649296e-3_f64) * t53692 * t4825 - F::cast_from(0.57165357490759649296e-3_f64) * t15707 * t16140 + F::cast_from(0.47637797908966374413e-4_f64) * t65581 - F::cast_from(0.45732285992607719436e-2_f64) * t43268 * t6308 + F::cast_from(0.57165357490759649296e-3_f64) * t65585 - F::cast_from(0.45732285992607719436e-2_f64) * t54739 * t1671 + F::cast_from(0.20325460441158986416e-2_f64) * t65589;
    t65591
}
