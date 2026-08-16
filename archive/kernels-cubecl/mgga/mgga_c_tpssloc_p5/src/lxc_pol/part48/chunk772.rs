//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 772/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk772<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23100: F, t23114: F, t23117: F, t23119: F, t23125: F, t23128: F, t23130: F, t23134: F, t23136: F, t23147: F) -> F {
    let t24218 = F::cast_from(0.10541775202358879834e-2_f64) * t23095;
    let t24220 = F::cast_from(0.33643963411783659044e-4_f64) * t23105;
    let t24221 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t23107;
    let t24230 = F::cast_from(0.22608743412718618878e-1_f64) * t23140;
    let t24231 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t23143;
    let t24233 = t24218 + F::cast_from(0.48447307312968469024e-2_f64) * t23100 - t24220 + t24221 + F::cast_from(0.13457585364713463618e-3_f64) * t23114 + t23117 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t23119 + F::cast_from(0.80745512188280781706e-3_f64) * t23125 - t23128 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t23130 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t23134 - t23136 / F::cast_from(192.0_f64) + t24230 + t24231 + t23147 / F::cast_from(96.0_f64);
    t24233
}
