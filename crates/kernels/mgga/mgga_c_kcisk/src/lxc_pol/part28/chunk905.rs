//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 905/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk905<F: Float>(t11612: F, t2063: F, t11634: F, t220: F, t1849: F, t3270: F, t1850: F, t6667: F, t6934: F, t965: F, t6937: F, t11625: F, t1049: F, t4597: F, t1809: F, t5101: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16227 = t11612 * t2063;
    let t16229 = t11634 * t220;
    let t16231 = t3270 * t1849;
    let t16232 = t16231 * t220;
    let t16246 = 0.47822877300252710492e-1 * t1850 * t6667;
    let t16251 = t965 * t6934;
    let t16254 = 0.17611111111111111111e-2 * t965 * t6937;
    let t16262 = 0.62154466893555682512e-3 * t11625 * t6667;
    let t16265 = t1049 * t4597;
    let t16298 = t1809 * t5101;
    (t16227, t16229, t16232, t16246, t16251, t16254, t16262, t16265, t16298)
}
