//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1321/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1321<F: Float>(t1220: F, t1308: F, t8020: F, t118919: F, t9426: F, t109701: F, t110558: F, t114011: F, t114062: F, t114095: F, t114205: F, t1312: F, t25313: F, t32019: F, t32096: F, t34707: F, t34803: F, t6204: F, t88072: F, t9429: F, t9446: F, t9447: F, t9454: F, t9796: F) -> (F, F) {
    let t119097 = t1220 * t8020 * t1308;
    let t119113 = t9426 * t118919;
    let t119123 = t114011 + 0.10416666666666666667e-1 * t119097 * t9429 + 0.20833333333333333334e-1 * t114205 * t9796 - 0.36848765432098765431e-3 * t109701 + 0.10416666666666666667e-1 * t32096 * t34707 + 0.10416666666666666667e-1 * t32019 * t34707 - 0.23148148148148148149e-2 * t114062 + 0.62500000000000000002e-1 * t9446 * t6204 * t110558 * t88072 - 0.46296296296296296296e-2 * t114095 - 0.40208333333333333333e-2 * t119113 + 0.10416666666666666667e-1 * t119097 * t9454 - 0.34722222222222222223e-2 * t9446 * t1312 * t9447 * t25313 - 0.46296296296296296297e-2 * t32096 * t34803;
    (t119097, t119123)
}
