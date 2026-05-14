//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 945/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk945<F: Float>(t1184: F, t3739: F, t852: F, t2240: F, t1185: F, t9976: F, t3033: F, t3766: F, t3769: F, t8219: F, t2242: F, t6142: F, t11155: F, t11185: F, t11187: F, t11191: F, t11196: F, t11198: F, t11200: F, t11207: F, t11211: F, t6161: F, t6175: F, t7950: F, t7955: F, t9782: F, t9819: F, t9826: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11233 = t3739 * t1184;
    let t11234 = t11233 * t852;
    let t11236 = 6.0 * t2240 * t11234;
    let t11238 = 3.0 * t9976 * t1185;
    let t11240 = 3.0 * t3033 * t3766;
    let t11242 = 0.48245938496077605201e2 * t8219 * t3769;
    let t11243 = t11233 * t2242;
    let t11245 = 0.96491876992155210402e2 * t6142 * t11243;
    let t11260 = 0.142419375e1 * t11185 - 0.28483875e1 * t11187 + 0.1898925e1 * t11191 - t6161 + 0.11958666666666666667e1 * t7955 - 0.89690000000000000001e0 * t9782 + 0.8969e0 * t11155 - 0.76790625e-1 * t11196 + 0.46074375e0 * t11198 + 0.3071625e0 * t11200 - t6175 + 0.82156666666666666666e0 * t7950 - 0.49293999999999999999e0 * t9819 - 0.49293999999999999999e0 * t9826 + 0.73941e0 * t11207 + 0.24647e0 * t11211;
    (t11233, t11234, t11236, t11238, t11240, t11242, t11243, t11245, t11260)
}
