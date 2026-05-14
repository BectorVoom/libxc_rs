//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1407/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1407<F: Float>(t1707: F, t2483: F, t5210: F, t7824: F, t5964: F, t1693: F, t22207: F, t22210: F, t22211: F, t22217: F, t22219: F, t22222: F, t22227: F, t22229: F, t22233: F, t22235: F) -> (F, F) {
    let t26622 = t2483 * t1707;
    let t26623 = 0.57791679765211885292e1 * t26622;
    let t26625 = t7824 * t5210;
    let t26627 = t7824 * t5964;
    let t26629 = t2483 * t1693;
    let t26630 = 0.3903689268108626343e0 * t26629;
    let t26637 = t26623 - 0.33872559466666666665e-2 * t22207 + 0.65061487801810439052e-1 * t26625 - 0.96319466275353142157e0 * t26627 - t22210 - t26630 - 0.3903689268108626343e0 * t22211 + 4.0 * t22217 + 0.34222787939297257218e3 * t22219 + 0.17337503929563565587e2 * t22222 + t22227 + 0.36018386108879999999e-1 * t22229 + 0.1800919305444e-1 * t22233;
    let t26638 = 48.0 * t22235;
    (t26637, t26638)
}
