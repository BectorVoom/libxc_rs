//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1971/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1971<F: Float>(t102346: F, t102361: F, t102363: F, t102364: F, t102367: F, t102372: F, t13747: F, t25921: F, t26282: F, t28850: F, t28899: F, t4131: F, t4132: F, t5728: F, t7295: F, t7296: F, t7511: F, t8085: F, t96380: F, t96382: F, t96398: F) -> F {
    let t102374 = -t102346 + F::cast_from(0.26341796731742046394e1_f64) * t7511 * t13747 - F::cast_from(0.65854491829355115987e0_f64) * t28899 * t4132 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t28850 + F::cast_from(0.34270468708064099208e-2_f64) * t96380 + F::cast_from(0.34270468708064099208e-2_f64) * t96382 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t8085 * t4131 + t102361 + t102363 - F::cast_from(0.22849835011101738147e-2_f64) * t102364 + t102367 + F::cast_from(0.26341796731742046394e1_f64) * t26282 * t5728 - t102372 - F::cast_from(0.48186823267806663678e-3_f64) * t96398;
    t102374
}
