//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 954/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk954(t112834: f64, t112840: f64, t112846: f64, t112850: f64, t112855: f64, t112823: f64, t112825: f64, t112827: f64, t112829: f64, t112832: f64, t112837: f64, t112843: f64, t112853: f64) -> f64 {
    let t114732 = 0.42167100809435519335e-2_f64 * t112834;
    let t114734 = 0.13457585364713463618e-3_f64 * t112840;
    let t114736 = 7.0_f64 / 576.0_f64 * t112846;
    let t114737 = 119.0_f64 / 3456.0_f64 * t112850;
    let t114739 = 0.90434973650874475512e-1_f64 * t112855;
    let t114740 = -t112823 / 192.0_f64 + 5.0_f64 / 192.0_f64 * t112825 - t112827 / 96.0_f64 + 0.22608743412718618878e-1_f64 * t112829 - 0.16149102437656156341e-2_f64 * t112832 + t114732 + 0.19378922925187387609e-1_f64 * t112837 - t114734 - 0.16149102437656156341e-2_f64 * t112843 - t114736 + t114737 + t112853 / 768.0_f64 + t114739;
    t114740
}
