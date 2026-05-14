//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1332/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1332<F: Float>(t108: F, t109: F, t111: F, t113: F, t1536: F, t1543: F, t20200: F, t24409: F, t25020: F, t25021: F, t25023: F, t25024: F, t25027: F, t25028: F, t25030: F, t2504: F, t25046: F, t2505: F, t2506: F, t25086: F, t2526: F, t486: F, t490: F, t491: F, t4933: F, t5052: F, t5054: F, t7165: F, t7175: F, t7185: F, t7188: F, t7191: F, t910: F, t95: F) -> (F,) {
    let t25088 = (-36.0 * t1536 * t95 * t2506 + 9.0 * t486 * t7191 + 180.0 * t2504 * t5052 * t2526 * t1543 - 72.0 * t7175 * t7185 - 36.0 * t7175 * t7188 - 12.0 * t2504 * t2505 * t4933 + 9.0 * t7165 * t491 - (t25020 + t25021 + t25023 + t25024 + t25027 + t25028 + t25030 + t25046) * t108 * t111 + 3.0 * t109 * t490 * t24409 - 360.0 * t2504 * t20200 * t910 * t5054 + t25086) * t113;
    (t25088,)
}
