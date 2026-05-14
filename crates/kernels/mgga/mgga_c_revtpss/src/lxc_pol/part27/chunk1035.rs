//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1035/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1035<F: Float>(t225: F, t26884: F, t494: F, t1210: F, t8945: F, t1248: F, t1287: F, t7638: F, t487: F, t7642: F, t7644: F, t3588: F, t7660: F, t11239: F, t1276: F, t2148: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26886 = t26884 * t225 * t494;
    let t26889 = t1210 * t8945;
    let t26891 = t7638 * t1248 * t1287;
    let t26894 = t7642 * t487;
    let t26895 = t26894 * t8945;
    let t26896 = t7644 * t1248;
    let t26897 = t26896 * t1287;
    let t26901 = t7660 * t3588 * t1287;
    let t26904 = t487 * t11239;
    let t26906 = t2148 * t26904 * t1276;
    (t26886, t26889, t26891, t26894, t26895, t26896, t26897, t26901, t26906)
}
