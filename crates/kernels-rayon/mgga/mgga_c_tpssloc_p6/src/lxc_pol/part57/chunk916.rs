//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 916/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk916(t114688: f64, t23012: f64, t8557: f64, t112834: f64, t112840: f64, t112850: f64, t112855: f64, t8538: f64, t2047: f64, t213: f64, t225: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114689 = 0.82246703342411321824e-2_f64 * t114688;
    let t114693 = t23012 * t8557;
    let t114694 = 0.63969658155208805863e-1_f64 * t114693;
    let t114732 = 0.42167100809435519335e-2_f64 * t112834;
    let t114734 = 0.13457585364713463618e-3_f64 * t112840;
    let t114737 = 119.0_f64 / 3456.0_f64 * t112850;
    let t114739 = 0.90434973650874475512e-1_f64 * t112855;
    let t114759 = t23012 * t8538;
    let t114760 = 0.63969658155208805863e-1_f64 * t114759;
    let t114770 = t213 * t2047 * t225;
    let t114790 = t794 * t2047;
    (t114689, t114694, t114732, t114734, t114737, t114739, t114760, t114770, t114790)
}
