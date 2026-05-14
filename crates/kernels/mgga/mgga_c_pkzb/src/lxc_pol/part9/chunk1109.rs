//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1109/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1109<F: Float>(t20759: F, t20762: F, t20765: F, t20769: F, t20773: F, t20777: F, t20781: F, t20789: F, t20791: F, t20794: F, t20797: F, t20800: F, t20803: F, t20806: F, t20809: F, t20811: F, t20813: F, t20815: F, t20817: F) -> (F,) {
    let t21082 = -0.62517e0 * t20759 - 0.125034e1 * t20762 - 0.62517e0 * t20765 + 0.312585e0 * t20769 + 0.937755e0 * t20773 + 0.937755e0 * t20777 + 0.312585e0 * t20781 + 0.6311625e0 * t20789 + 0.3529725e1 * t20791 + 0.794188125e1 * t20794 - 0.473371875e0 * t20797 - 0.6618234375e1 * t20800 + 0.2366859375e0 * t20803 + 0.94674375e0 * t20806 - 0.52945875e1 * t20809 - 0.52945875e1 * t20811 - 0.17648625e1 * t20813 + 0.94674375e0 * t20815 + 0.31558125e0 * t20817;
    (t21082,)
}
