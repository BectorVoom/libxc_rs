//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1220/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1220<F: Float>(t34125: F, t34135: F, t36824: F, t36825: F, t36826: F, t36827: F, t36828: F, t36829: F, t36830: F, t36832: F, t36833: F, t34176: F, t34178: F, t34193: F, t34200: F, t36849: F, t36850: F, t36851: F, t36854: F, t36855: F, t36856: F, t36857: F) -> (F, F) {
    let t38795 = t36824 + t36825 + t36826 - t36827 + t36828 - t36829 - t36830 + 0.95956020918421216159e-7 * t34125 + t36832 - t36833 + 0.25301106770833333334e-5 * t34135;
    let t38805 = t36849 + t36850 + t36851 + 0.36231816839129402172e-6 * t34176 + 0.72463633678258804344e-6 * t34178 - t36854 + t36855 + t36856 - t36857 + 0.7379489474826388889e-6 * t34193 - 0.38527756621470067413e-7 * t34200;
    (t38795, t38805)
}
