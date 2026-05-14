//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1107/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1107<F: Float>(t10003: F, t94508: F, t25997: F, t9970: F, t550: F, t7021: F, t3946: F, t1412: F, t1941: F, t9750: F, t25273: F, t540: F, t1372: F, t2019: F, t9951: F, t2018: F, t9646: F, t9723: F) -> (F, F, F, F, F, F, F) {
    let t94509 = t94508 * t10003;
    let t94511 = t25997 * t9970;
    let t94513 = t7021 * t550;
    let t94514 = t94513 * t3946;
    let t94516 = t1941 * t1412;
    let t94517 = t94516 * t9750;
    let t94519 = t25273 * t540;
    let t94520 = t94519 * t1372;
    let t94522 = t2019 * t9951;
    let t94523 = 0.7558530601555998074e-1 * t94522;
    let t94525 = t9646 * t2018 * t9723;
    (t94509, t94511, t94514, t94517, t94520, t94523, t94525)
}
