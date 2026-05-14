//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 771/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk771<F: Float>(t4012: F, t828: F, t9984: F, t1384: F, t235: F, t239: F, t820: F, t4003: F, t543: F, t9898: F, t1390: F, t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F) -> (F, F, F, F, F, F, F, F) {
    let t9986 = t4012 * t828 * t9984;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0 / t9989;
    let t9991 = t9990 * t235;
    let t9993 = t820 * t9991 * t239;
    let t9994 = t4003 * t543;
    let t9995 = t9898 * t9994;
    let t9997 = t1390 * t828 * t9995;
    let t10001 = t2482 * t4000 * t27;
    let t10003 = t4019 * t221 * t4004;
    (t9986, t9990, t9993, t9994, t9995, t9997, t10001, t10003)
}
