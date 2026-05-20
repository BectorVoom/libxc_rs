//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2869/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2869<F: Float>(t231: F, t2782: F, t2783: F, t76127: F, t18615: F, t18632: F, t18677: F, t2723: F, t40314: F, t40316: F, t4494: F, t4504: F, t51396: F, t51513: F, t6022: F, t62840: F, t62843: F, t62847: F, t62853: F, t820: F) -> F {
    let t77197 = t2782 * t2783 * t76127 * t231;
    let t77213 = F::cast_from(0.54878743191129263322e-2_f64) * t77197 + F::cast_from(0.32927245914677557992e-1_f64) * t62840 + F::cast_from(0.11853808529283920877e2_f64) * t4504 * t18677 * t18632 + F::cast_from(0.21951497276451705328e-1_f64) * t62843 - t40314 + t40316 + t51513 - F::cast_from(0.19514881078765566037e-2_f64) * t62847 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t4494 * t2723 * t18615 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t51396 * t6022 - F::cast_from(0.65854491829355115984e-1_f64) * t62853;
    t77213
}
