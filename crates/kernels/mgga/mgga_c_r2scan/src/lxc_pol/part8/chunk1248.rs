//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1248/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1248<F: Float>(t3216: F, t6212: F, t6211: F, t6475: F, t25983: F, t8157: F, t2236: F, t9452: F, t20758: F, t3187: F, t25755: F, t7262: F, t3016: F, t6480: F, t5146: F, t5148: F, t625: F, t9422: F) -> (F, F, F, F, F, F, F) {
    let t27955 = t6212 * t3216;
    let t27957 = t6475 * t6211 * t27955;
    let t27959 = t25983 * t8157;
    let t27966 = t2236 * t9452;
    let t27973 = t20758 * t3187;
    let t27975 = t25755 * t7262;
    let t27977 = t6212 * t3016;
    let t27979 = t6480 * t6211 * t27977;
    let t27983 = t5146 * t5148 * t9422 * t625;
    (t27957, t27959, t27966, t27973, t27975, t27979, t27983)
}
