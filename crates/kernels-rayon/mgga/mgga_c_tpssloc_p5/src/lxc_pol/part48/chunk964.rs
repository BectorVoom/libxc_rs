//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 964/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk964(t23030: f64, t31319: f64, t23168: f64, t31367: f64, t112893: f64, t112902: f64, t112905: f64, t114741: f64, t114877: f64, t114880: f64, t114882: f64, t114889: f64, t1912: f64, t2054: f64, t218: f64, t23215: f64, t259: f64, t2713: f64, t31311: f64, t7087: f64, t82071: f64, t85079: f64, t85146: f64, t85152: f64, t8563: f64, t9593: f64) -> f64 {
    let t114891 = t23030 * t31319;
    let t114892 = 0.26044789391763585244e-1_f64 * t114891;
    let t114900 = t23168 * t31367;
    let t114902 = -t85079 * t1912 - 2.0_f64 * t9593 * t8563 - t82071 * t2054 - 0.6579736267392905746e-1_f64 * t114877 + 0.3289868133696452873e-1_f64 * t114880 + 0.38381794893125283518e-1_f64 * t114882 - 2.0_f64 * t85146 * t1912 - t112893 + 0.82246703342411321825e-2_f64 * t114889 + t114892 + t218 * t114741 * t259 - t85152 * t1912 + t112902 + 4.0_f64 * t2713 * t31311 - 6.0_f64 * t7087 * t23215 + t112905 + 0.76763589786250567036e-1_f64 * t114900;
    t114902
}
