//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 914/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk914(t2268: f64, t2765: f64, t9152: f64, t39791: f64, t39794: f64, t39798: f64, t42778: f64, t42782: f64, t42786: f64, t42790: f64, t42793: f64, t42795: f64, t42797: f64, t42799: f64, t42802: f64, t42803: f64, t42804: f64, t42806: f64, t42808: f64, t42811: f64) -> f64 {
    let t42814 = 0.85365019907028448797e-1_f64 * t2268 * t2765 * t9152;
    let t42815 = 0.23712505529730124666e-2_f64 * t39791;
    let t42816 = 0.23712505529730124666e-2_f64 * t39794;
    let t42817 = 0.23712505529730124666e-2_f64 * t39798;
    let t42818 = -0.3983700928994660944e0_f64 * t42778 + 0.6829201592562275904e0_f64 * t42782 - 0.3414600796281137952e0_f64 * t42786 + t42790 + t42793 - t42795 - t42797 - t42799 - t42802 - t42803 + t42804 + t42806 - t42808 - t42811 - t42814 + t42815 + t42816 - t42817;
    t42818
}
