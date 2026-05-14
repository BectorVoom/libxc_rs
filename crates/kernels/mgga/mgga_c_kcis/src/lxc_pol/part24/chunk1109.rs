//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1109/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1109<F: Float>(t11081: F, t26960: F, t29116: F, t1092: F, t1121: F, t27763: F, t6696: F, t100056: F, t100321: F, t18508: F, t26955: F, t27014: F, t28116: F, t29094: F, t5310: F, t7788: F, t95817: F, t96885: F, t96899: F, t96902: F, t96904: F) -> (F, F) {
    let t100330 = t26960 * t11081 * t29116;
    let t100340 = t1092 * t27763 * t6696 * t1121;
    let t100349 = 0.7722800925925925926e-4 * t100330 - 0.30918233506944444445e-4 * t26955 * t100321 + 0.23168402777777777778e-3 * t26960 * t5310 * t28116 * t18508 + 0.34822083333333333332e-2 * t100340 - t96885 - 0.51588271604938271603e-3 * t95817 + 0.34752604166666666667e-3 * t7788 * t100056 - 0.82448622685185185184e-4 * t96899 - 0.69505208333333333334e-3 * t27014 * t29094 - 0.15445601851851851852e-3 * t96902 + t96904;
    (t100340, t100349)
}
