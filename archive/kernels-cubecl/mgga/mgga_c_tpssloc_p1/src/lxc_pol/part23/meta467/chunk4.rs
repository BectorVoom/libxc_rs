//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1372/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372<F: Float>(t41962: F, t47787: F, t76587: F, t76595: F, t76610: F, t76618: F, t76626: F, t76899: F, t76903: F, t76906: F, t76912: F, t77102: F, t77105: F, t77107: F) -> F {
    let t77218 = t41962 - F::cast_from(0.22076e0_f64) * t76899 + F::cast_from(0.66228e0_f64) * t76903 - F::cast_from(0.11038e0_f64) * t76906 - F::cast_from(0.298026e1_f64) * t76912 + F::cast_from(0.258925e1_f64) * t77102 + F::cast_from(0.12524296296296296297e1_f64) * t47787 - F::cast_from(0.3883875e1_f64) * t77105 + F::cast_from(0.6189328125e-1_f64) * t77107 - F::cast_from(0.20128333333333333334e1_f64) * t76587 + F::cast_from(0.72462e1_f64) * t76595 - F::cast_from(0.80513333333333333332e0_f64) * t76610 - F::cast_from(0.108693e2_f64) * t76618 + F::cast_from(0.24154e1_f64) * t76626;
    t77218
}
