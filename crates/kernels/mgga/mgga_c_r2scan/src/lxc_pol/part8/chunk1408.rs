//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1408/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1408<F: Float>(t44: F, t10308: F, t133: F, t255: F, t546: F, t565: F, t1217: F, t2509: F, t2512: F, t3000: F, t32155: F, t32158: F, t32168: F, t415: F, t8571: F, t903: F, t9859: F, t9865: F, t99: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t34140 = t133 * t10308 * t255;
    let t34141 = t546 * t34140;
    let t34144 = t565 * t34140;
    let t34162 = piecewise3(t45, 0.0, 40.0 / 81.0 * t9859 * t415 - 20.0 / 9.0 * t3000 * t1217 - 10.0 / 9.0 * t2509 * t32155 + 20.0 / 3.0 * t2512 * t32158 + 10.0 / 3.0 * t903 * t8571 + 10.0 / 9.0 * t9865 * t415 + 5.0 / 3.0 * t99 * t32168);
    (t34141, t34144, t34162)
}
