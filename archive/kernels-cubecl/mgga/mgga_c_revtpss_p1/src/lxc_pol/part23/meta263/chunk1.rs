//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1468/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1468<F: Float>(t10022: F, t786: F, t2435: F, t4093: F, t4083: F, t9303: F, t2777: F, t4092: F, t2439: F, t1419: F, t3999: F, t123: F, t212: F, t2434: F) -> (F, F, F, F, F, F, F) {
    let t10023 = t786 * t10022;
    let t10032 = t2435 * t4093;
    let t10035 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t4083;
    let t10043 = t2777 * t4092;
    let t10044 = t2439 * t10043;
    let t10049 = t3999 * t1419;
    let t10069 = t123 * t2434 * t212;
    (t10023, t10032, t10035, t10043, t10044, t10049, t10069)
}
