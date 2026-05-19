//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1017/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1017<F: Float>(t11209: F, t218: F, t219: F, t11155: F, t11185: F, t11187: F, t11191: F, t11196: F, t11198: F, t11200: F, t11207: F, t6211: F, t6218: F, t7950: F, t7955: F, t9782: F, t9819: F, t9826: F) -> (F, F) {
    let t11211 = t218 * t219 * t11209;
    let t11213 = F::new(0.19419375e1) * t11185 - F::new(0.3883875e1) * t11187 + F::new(0.258925e1) * t11191 - t6211 + F::new(0.12077e1) * t7955 - F::new(0.905775e0) * t9782 + F::new(0.905775e0) * t11155 - F::cast_from(0.412621875e-1_f64) * t11196 + F::cast_from(0.247573125e0_f64) * t11198 + F::new(0.16504875e0) * t11200 - t6218 + F::new(0.82785e0) * t7950 - F::new(0.49671e0) * t9819 - F::new(0.49671e0) * t9826 + F::new(0.745065e0) * t11207 + F::new(0.248355e0) * t11211;
    (t11211, t11213)
}
