//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1030/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1030<F: Float>(t3262: F, t3276: F, t42855: F, t11506: F, t40421: F, t11523: F, t11545: F, t35373: F, t481: F, t39263: F, t4176: F, t2449: F, t3696: F, t42818: F, t42822: F, t42824: F, t42826: F, t42832: F, t42836: F, t42840: F, t42843: F, t42845: F, t42850: F, t42854: F) -> (F, F, F, F, F) {
    let t42858 = 15.0 / 16.0 * t3262 * t3276 * t42855;
    let t42860 = 3.0 / 2.0 * t11506 * t40421;
    let t42862 = 5.0 / 8.0 * t11523 * t11545;
    let t42863 = t35373 * t481;
    let t42866 = 3.0 * t39263 * t4176 * t42863;
    let t42867 = 2.0 * t2449 * t3696 + t42818 + t42822 + t42824 - t42826 + t42832 + t42836 + t42840 - t42843 - t42845 - t42850 + t42854 - t42858 + t42860 - t42862 - t42866;
    (t42858, t42860, t42862, t42866, t42867)
}
