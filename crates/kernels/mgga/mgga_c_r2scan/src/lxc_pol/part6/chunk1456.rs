//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1456/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1456<F: Float>(t19061: F, t19069: F, t19341: F, t19388: F, t23778: F, t23779: F, t23780: F, t23781: F, t23794: F, t23796: F, t23798: F, t19394: F, t19405: F, t23801: F, t23802: F, t23803: F, t23810: F, t23813: F, t23816: F, t23819: F, t23823: F, t23829: F) -> (F, F) {
    let t27435 = t23778 + t19061 - t23779 + t23780 + t19069 + t23781 - t19341 - t23794 - t23796 - t23798 - t19388;
    let t27439 = -t19394 - t23801 + t23802 - t23803 + t19405 - t23810 + t23813 + t23816 + t23819 + t23823 + t23829;
    (t27435, t27439)
}
