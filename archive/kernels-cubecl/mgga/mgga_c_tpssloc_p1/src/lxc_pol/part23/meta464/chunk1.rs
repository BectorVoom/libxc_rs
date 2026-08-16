//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1359/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1359<F: Float>(t4475: F, t68902: F, t959: F, t17934: F, t5812: F, t21370: F, t4483: F, t76665: F, t76668: F, t76671: F, t76674: F, t76997: F, t77001: F, t77003: F, t77006: F, t77009: F) -> (F, F, F, F) {
    let t77012 = F::cast_from(0.69263436422725855036e2_f64) * t959 * t68902 * t4475;
    let t77014 = F::cast_from(0.10389515463408878255e3_f64) * t17934 * t5812;
    let t77016 = F::cast_from(0.20779030926817756511e3_f64) * t4483 * t21370;
    let t77017 = t76665 + t76668 - t76671 + t76674 - t76997 + t77001 + t77003 + t77006 + t77009 - t77012 - t77014 - t77016;
    (t77012, t77014, t77016, t77017)
}
