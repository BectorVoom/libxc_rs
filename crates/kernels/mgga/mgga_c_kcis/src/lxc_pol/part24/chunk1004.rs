//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1004/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1004<F: Float>(t26960: F, t26974: F, t27077: F, t27849: F, t28161: F, t28190: F, t28204: F, t28920: F, t29094: F, t29123: F, t29127: F, t7772: F, t7788: F, t8087: F, t8095: F, t27028: F, t6774: F) -> (F, F) {
    let t29143 = 0.23168402777777777778e-3 * t26960 * t29123 + 0.34752604166666666667e-3 * t7788 * t29127 - 0.92835860883789062501e-5 * t27077 * t29094 - 0.13913205078125e-3 * t7772 * t29094 + t26974 + 0.17411041666666666666e-2 * t28920 + 0.23168402777777777778e-3 * t28161 + 0.15476481481481481481e-2 * t27849 + 0.69505208333333333334e-3 * t28190 * t8095 + 0.69505208333333333334e-3 * t28190 * t8087 + 0.92754700520833333334e-4 * t28204 * t8087;
    let t29147 = t27028 * t6774;
    (t29143, t29147)
}
