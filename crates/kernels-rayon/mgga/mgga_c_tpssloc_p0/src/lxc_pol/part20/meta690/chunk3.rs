//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2621/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2621(t11148: f64, t11665: f64, t11674: f64, t11678: f64, t11724: f64, t11766: f64, t11855: f64, t1216: f64, t14706: f64, t15470: f64, t15661: f64, t15663: f64, t15740: f64, t1735: f64, t18946: f64, t3577: f64, t3578: f64, t3580: f64, t45128: f64, t45162: f64, t45211: f64, t4889: f64, t5019: f64, t53322: f64, t53336: f64) -> f64 {
    let t53345 = -5.0_f64 / 5184.0_f64 * t3577 * t45128 * t1735 * t11148 + 7.0_f64 / 243.0_f64 * t4889 * t11766 + 5.0_f64 / 6912.0_f64 * t45211 - t53322 * t3580 / 768.0_f64 - t15740 * t11674 / 1536.0_f64 - t11665 * t15470 / 768.0_f64 - t3577 * t3578 * t14706 * t1216 / 1536.0_f64 - t5019 * t11855 / 576.0_f64 - t53336 * t11724 / 96.0_f64 - t45162 * t15663 / 384.0_f64 - t11678 * t3578 * t18946 * t15661 / 384.0_f64;
    t53345
}
