//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1442/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1442<F: Float>(t11292: F, t11433: F, t1164: F, t3377: F, t11285: F, t43679: F, t44154: F, t11923: F, t225: F, t10913: F, t11583: F, t11570: F) -> (F, F, F, F, F) {
    let t44396 = F::cast_from(0.62337092780453269531e3_f64) * t1164 * t11292 * t3377 * t11433;
    let t44400 = F::cast_from(0.12304822629859687989e5_f64) * t1164 * t44154 * t43679 * t11285;
    let t44412 = t11923 * t225;
    let t44415 = t11583 * t10913;
    let t44419 = t11570 * t10913;
    (t44396, t44400, t44412, t44415, t44419)
}
