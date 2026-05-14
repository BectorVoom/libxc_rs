//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1133/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1133<F: Float>(t114: F, t2339: F, t624: F, t2340: F, t2366: F, t25823: F, t10208: F, t68: F, t10209: F, t665: F, t25826: F, t10254: F, t6998: F, t94974: F, t94976: F, t1312: F, t2014: F, t26089: F, t7315: F) -> (F, F, F) {
    let t115 = 1.0 < t114;
    let t94978 = t624 * t2339;
    let t94979 = t94978 * t2340;
    let t94981 = t25823 * t2366;
    let t94982 = t68 * t10208;
    let t94983 = t94982 * t10209;
    let t94985 = t665 * t2366;
    let t94986 = t25826 * t94985;
    let t94988 = t6998 * t10254;
    let t94991 = piecewise3(t115, 0.0, -t94974 - 11.0 / 3.0 * t94976 - 2.0 * t94979 + t94981 - 3.0 / 4.0 * t94983 + 3.0 / 4.0 * t94986 - t94988 / 8.0);
    let t94993 = 2.0 * t1312 * t94991;
    let t94998 = 3.0 * t2014 * t26089 * t7315;
    (t94991, t94993, t94998)
}
