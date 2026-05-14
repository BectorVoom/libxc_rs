//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1415/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1415<F: Float>(t25983: F, t9381: F, t8825: F, t910: F, t2148: F, t6165: F, t3071: F, t1568: F, t6155: F, t2155: F, t32669: F, t6063: F, t33288: F, t6086: F, t6093: F, t32871: F) -> (F, F, F, F, F, F, F) {
    let t34339 = t25983 * t9381;
    let t34341 = t8825 * t910;
    let t34343 = t6165 * t2148 * t34341;
    let t34345 = t3071 * t910;
    let t34347 = t6155 * t1568 * t34345;
    let t34351 = t2155 * t6063 * t32669;
    let t34354 = t6093 * t6086 * t33288;
    let t34357 = t2155 * t6063 * t32871;
    (t34339, t34343, t34345, t34347, t34351, t34354, t34357)
}
