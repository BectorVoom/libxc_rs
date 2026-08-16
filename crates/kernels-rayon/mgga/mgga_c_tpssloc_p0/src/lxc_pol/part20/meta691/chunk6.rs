//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2629/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2629(t11761: f64, t1232: f64, t14725: f64, t3577: f64, t45128: f64, t45256: f64, t45260: f64, t45262: f64, t4889: f64, t52538: f64, t53481: f64, t53487: f64, t53490: f64, t53494: f64, t53496: f64, t53498: f64) -> f64 {
    let t53503 = -t53481 / 576.0_f64 - 5.0_f64 / 1728.0_f64 * t3577 * t45128 * t14725 * t52538 - t53487 * t1232 / 1536.0_f64 - 5.0_f64 / 486.0_f64 * t53490 - 2.0_f64 / 27.0_f64 * t4889 * t11761 - t53494 / 1152.0_f64 + t53496 / 108.0_f64 + t53498 / 54.0_f64 + 5.0_f64 / 6912.0_f64 * t45256 + 5.0_f64 / 3456.0_f64 * t45260 + t45262 / 1536.0_f64;
    t53503
}
