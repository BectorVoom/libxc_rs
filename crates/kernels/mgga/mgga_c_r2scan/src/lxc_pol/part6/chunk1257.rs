//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1257/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1257<F: Float>(t322: F, t1018: F, t1305: F, t1307: F, t23538: F, t23594: F, t23635: F, t23681: F, t2405: F, t330: F, t6701: F, t6706: F, t837: F, t838: F, t8420: F, t8425: F, t18850: F) -> (F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t23682 = piecewise5(t323, t1018 * t330 * t6701 + t1018 * t330 * t6706 + 3.0 * t1305 * t2405 * t330 + 3.0 * t1307 * t2405 * t330 + 3.0 * t330 * t837 * t8420 + t23538 * t330 + 3.0 * t838 * t8425, t331, t23594 + t23635, t23681);
    let t23685 = 4.0 * t18850;
    (t23682, t23685)
}
