//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1259/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1259<F: Float>(t1882: F, t26334: F, t47660: F, t5630: F, t100: F, t7241: F, t26195: F, t100076: F, t102372: F, t102392: F, t10974: F, t11064: F, t11397: F, t11593: F, t11618: F, t11837: F, t11859: F, t11871: F, t12026: F, t1901: F, t1909: F, t23084: F, t23294: F, t23327: F, t26171: F, t3052: F, t446: F, t452: F, t47659: F, t47666: F, t488: F, t5717: F, t5722: F, t83: F, t91739: F, t942: F) -> (F,) {
    let t103745 = 2.0 / 27.0 * t1882 * t26334;
    let t103753 = t47660 * t5630;
    let t103761 = t7241 * t100 * t5630;
    let t103769 = 2.0 / 9.0 * t1882 * t26195;
    let t103775 = 2.0 / 3.0 * t446 * t452 * t11837 * t5722 - t446 * t83 * t102372 / 3.0 + 2.0 / 3.0 * t446 * t83 * t102392 + 2.0 * t1901 * t26171 * t5717 * t11618 + 2.0 / 9.0 * t1901 * t23327 * t12026 + 2.0 / 3.0 * t446 * t83 * t100076 + t103745 + 4.0 / 9.0 * t11593 * t1909 * t23294 * t3052 + 4.0 / 9.0 * t47659 * t91739 * t11859 + 8.0 / 9.0 * t47659 * t103753 * t11397 - 8.0 / 27.0 * t47666 * t103753 * t10974 + 4.0 / 3.0 * t47659 * t103761 * t11064 + 4.0 / 9.0 * t47659 * t91739 * t11871 + t103769 + t446 * t452 * t488 * t23084 * t942 / 3.0;
    (t103775,)
}
