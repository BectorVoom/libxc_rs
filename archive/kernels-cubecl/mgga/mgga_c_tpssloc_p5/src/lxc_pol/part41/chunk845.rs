//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 845/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk845<F: Float>(t1174: F, t1726: F, t1737: F, t3577: F, t488: F, t4889: F, t4957: F, t4959: F, t4994: F, t4998: F, t5002: F, t6158: F, t6165: F, t6170: F, t6178: F, t6184: F, t6188: F, t6192: F) -> F {
    let t6197 = -t6158 * t488 / F::cast_from(288.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1728.0_f64) * t6165 * t488 + t6170 * t488 / F::cast_from(3072.0_f64) + t4957 / F::cast_from(2304.0_f64) - t4959 / F::cast_from(432.0_f64) - t4994 / F::cast_from(3456.0_f64) + t4998 / F::cast_from(2304.0_f64) + t1174 * t6178 / F::cast_from(216.0_f64) + t4889 * t1726 / F::cast_from(54.0_f64) - t1174 * t6184 / F::cast_from(288.0_f64) - t1174 * t6188 / F::cast_from(144.0_f64) - t3577 * t6192 / F::cast_from(2304.0_f64) + t5002 * t1737 / F::cast_from(1536.0_f64);
    t6197
}
