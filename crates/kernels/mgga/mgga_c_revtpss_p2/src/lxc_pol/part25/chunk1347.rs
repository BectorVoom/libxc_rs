//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1347/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1347<F: Float>(t114: F, t94974: F, t94976: F, t94979: F, t94981: F, t94983: F, t94986: F, t94988: F, t1312: F, t10259: F, t2371: F, t25805: F, t28025: F, t670: F, t6985: F, t92719: F, t92737: F, t94947: F, t94956: F, t94958: F, t94960: F, t94962: F, t94964: F, t94966: F, t94968: F, t94970: F, t94972: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t94991 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t94974 - F::cast_from(11.0_f64) / F::cast_from(3.0_f64) * t94976 - F::cast_from(2.0_f64) * t94979 + t94981 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t94983 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t94986 - t94988 / F::cast_from(8.0_f64));
    let t94993 = F::cast_from(2.0_f64) * t1312 * t94991;
    let t94994 = F::cast_from(2.0_f64) * t10259 * t6985 + F::cast_from(6.0_f64) * t2371 * t25805 + F::cast_from(6.0_f64) * t2371 * t28025 + F::cast_from(6.0_f64) * t670 * t92737 + t92719 + F::cast_from(6.0_f64) * t94947 + t94956 + t94958 + t94960 + t94962 + t94964 + t94966 + t94968 + t94970 + t94972 + t94993;
    (t94991, t94994)
}
