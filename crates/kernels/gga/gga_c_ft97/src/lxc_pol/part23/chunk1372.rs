//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1372/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1372<F: Float>(t123759: F, t1701: F, t811: F, t820: F, t19100: F, t4061: F, t2035: F, t4088: F, t6979: F, t112220: F, t112366: F, t112367: F, t127434: F, t127519: F, t127530: F, t127534: F, t127537: F, t127539: F, t127543: F, t127545: F, t127548: F, t1472: F, t14766: F, t19101: F, t19107: F, t19132: F, t28591: F, t31440: F, t31489: F, t31502: F, t31530: F, t4065: F, t4094: F, t4104: F, t82957: F) -> (F, F, F, F) {
    let t127553 = t1701 * t123759 * t811;
    let t127557 = t1701 * t123759 * t820;
    let t127560 = t4061 * t19100;
    let t127564 = t2035 * t6979 * t4088;
    let t127567 = -0.48327307107230638237e1 * t28591 * t31489 - 0.48327307107230638237e1 * t4094 * t127519 + 0.90613700826057446696e0 * t112220 * t31530 + 0.90613700826057446696e0 * t14766 * t127434 + 0.10947790369858991998e1 * t112366 * t112367 * t4065 - 0.3722248725752057279e2 * t19101 * t127530 + 0.18611243628760286395e2 * t19107 * t127534 + 0.11300578175490223805e0 * t127537 * t127539 - 0.11300578175490223805e0 * t127543 * t127545 - 0.94171484795751865043e-2 * t127548 - 0.10947790369858991998e1 * t82957 * t31440 - 0.15303647250623035441e2 * t4104 * t127553 + 0.76518236253115177207e1 * t1472 * t127557 - 0.21895580739717983994e1 * t127560 * t31502 - 0.21895580739717983994e1 * t19132 * t127564;
    (t127553, t127557, t127564, t127567)
}
