//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2010/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2010<F: Float>(t1068: F, t4696: F, t1597: F, t976: F, t1022: F, t3966: F, t1395: F, t671: F, t23862: F, t580: F, t23901: F, t576: F) -> (F, F, F, F, F, F) {
    let t60941 = t4696 * t1068;
    let t61066 = t976 * t1597;
    let t61774 = t3966 * t1022;
    let t66940 = t1395 * t671;
    let t80593 = t23862 * t580;
    let t80597 = t576 * t23901;
    (t60941, t61066, t61774, t66940, t80593, t80597)
}
