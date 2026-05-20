//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1608/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1608<F: Float>(t39554: F, t39557: F, t50205: F, t50214: F, t61361: F, t61367: F, t61371: F, t61397: F, t61400: F, t61407: F, t61411: F, t75974: F, t75978: F, t75984: F, t75998: F, t76010: F) -> F {
    let t87357 = F::cast_from(0.13170898365871023197e0_f64) * t75974 + t39554 - F::cast_from(0.23417857294518679245e0_f64) * t75978 + t39557 - F::cast_from(0.43902994552903410657e-1_f64) * t61361 + F::cast_from(0.87805989105806821314e-1_f64) * t61367 + F::cast_from(0.69394917116090352835e-2_f64) * t61371 + F::cast_from(0.23417857294518679245e0_f64) * t75984 - F::cast_from(0.12142592671231907757e0_f64) * t50205 - F::cast_from(0.13170898365871023197e0_f64) * t75998 - F::cast_from(0.18505311230957427423e-1_f64) * t50214 - F::cast_from(0.7805952431506226415e-2_f64) * t61397 + F::cast_from(0.7805952431506226415e-2_f64) * t61400 + F::cast_from(0.13878983423218070567e-1_f64) * t61407 - F::cast_from(0.11708928647259339623e0_f64) * t76010 - F::cast_from(0.69394917116090352835e-2_f64) * t61411;
    t87357
}
