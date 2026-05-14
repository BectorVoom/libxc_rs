//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1340/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1340<F: Float>(t34403: F, t5014: F, t34411: F, t9724: F, t116736: F, t116747: F, t2020: F, t2029: F, t7233: F, t10005: F, t33234: F, t34465: F, t9721: F, t5439: F, t786: F, t33162: F, t34444: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t118103 = t5014 * t34403;
    let t118120 = t9724 * t34411;
    let t118129 = 0.23214722222222222222e-2 * t116736;
    let t118132 = 0.23214722222222222222e-2 * t116747;
    let t118141 = t2020 * t2029;
    let t118142 = t7233 * t118141;
    let t118150 = t10005 * t33234;
    let t118174 = 0.34722222222222222222e-2 * t9721 * t34465;
    let t118180 = t5014 * t118141;
    let t118184 = t786 * t5439;
    let t118185 = t5014 * t118184;
    let t118206 = 0.13402777777777777778e-2 * t34444 * t33162;
    (t118103, t118120, t118129, t118132, t118142, t118150, t118174, t118180, t118184, t118185, t118206)
}
