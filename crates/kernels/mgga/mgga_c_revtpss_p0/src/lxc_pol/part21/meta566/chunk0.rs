//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2264/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2264<F: Float>(t1121: F, t1248: F, t606: F, t3604: F, t17353: F, t372: F, t5277: F, t3630: F, t12784: F, t12866: F, t12910: F, t17619: F, t17622: F, t17625: F, t17629: F, t17635: F, t17640: F, t17646: F, t17651: F, t17654: F, t3625: F, t5402: F) -> (F, F, F, F, F, F) {
    let t17655 = t1248 * t1121;
    let t17656 = t17655 * t606;
    let t17657 = t3604 * t17656;
    let t17658 = t17353 * t17657;
    let t17661 = t372 * t5277;
    let t17662 = t17661 * t3630;
    let t17665 = -t17619 - t17622 + F::cast_from(0.42874018118069736972e-3_f64) * t12910 * t17625 + t17629 / F::new(1296.0) - F::cast_from(0.28582678745379824648e-3_f64) * t12784 * t5402 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t17635 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t17640 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t17646 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17651 - F::cast_from(0.57165357490759649296e-3_f64) * t17654 * t17658 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17662;
    (t17655, t17657, t17658, t17661, t17662, t17665)
}
