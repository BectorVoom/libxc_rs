//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1178/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1178<F: Float>(t34119: F, t34121: F, t34127: F, t34132: F, t34125: F, t34135: F, t36824: F, t36825: F, t36826: F, t36827: F, t36828: F, t34142: F, t34144: F, t34146: F, t34148: F, t34154: F) -> (F, F, F, F, F, F) {
    let t36829 = 0.70341874126922921074e-8 * t34119;
    let t36830 = 0.70341874126922921074e-8 * t34121;
    let t36832 = 0.34179092986183952014e-5 * t34127;
    let t36833 = 0.24581606547037760418e-8 * t34132;
    let t36835 = t36824 + t36825 + t36826 - t36827 + t36828 - t36829 - t36830 + 0.95956020918421216158e-7 * t34125 + t36832 - t36833 + 0.25301106770833333336e-5 * t34135;
    let t36838 = 0.50680539737635041234e-3 * t34142;
    let t36839 = 0.20240885416666666668e-4 * t34144;
    let t36840 = 0.20240885416666666668e-3 * t34146;
    let t36841 = 0.40481770833333333336e-4 * t34148;
    let t36843 = 0.40481770833333333336e-4 * t34154;
    (t36835, t36838, t36839, t36840, t36841, t36843)
}
