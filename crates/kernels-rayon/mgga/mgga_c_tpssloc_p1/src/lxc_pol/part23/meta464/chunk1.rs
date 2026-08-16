//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1359/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1359(t4475: f64, t68902: f64, t959: f64, t17934: f64, t5812: f64, t21370: f64, t4483: f64, t76665: f64, t76668: f64, t76671: f64, t76674: f64, t76997: f64, t77001: f64, t77003: f64, t77006: f64, t77009: f64) -> (f64, f64, f64, f64) {
    let t77012 = 0.69263436422725855036e2_f64 * t959 * t68902 * t4475;
    let t77014 = 0.10389515463408878255e3_f64 * t17934 * t5812;
    let t77016 = 0.20779030926817756511e3_f64 * t4483 * t21370;
    let t77017 = t76665 + t76668 - t76671 + t76674 - t76997 + t77001 + t77003 + t77006 + t77009 - t77012 - t77014 - t77016;
    (t77012, t77014, t77016, t77017)
}
