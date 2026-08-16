//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 681/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk681<F: Float>(t381: F, t5848: F, t1603: F, t1625: F, t1044: F, t248: F, t5685: F, t3062: F, t5677: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F) -> (F, F, F, F, F) {
    let t5849 = t5848 * t381;
    let t5851 = t1603 * t1625;
    let t5857 = t248 * t1044 * t5685;
    let t5861 = t248 * t3062 * t5677;
    let t5866 = -t5691 + t5693 - t5697 + t5729 + t5732 + t5798 + t5800 - t5802 + t5806 - t5810 - t5814;
    (t5849, t5851, t5857, t5861, t5866)
}
