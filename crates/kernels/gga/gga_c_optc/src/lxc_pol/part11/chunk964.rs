//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 964/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk964<F: Float>(t1506: F, t4573: F, t3119: F, t4374: F, t1111: F, t12026: F, t1509: F, t1520: F, t15225: F, t15228: F, t15255: F, t15272: F, t15327: F, t15355: F, t17663: F, t17667: F, t17670: F, t17674: F, t17677: F, t3116: F, t4363: F, t4369: F, t5314: F, t5325: F, t5337: F) -> (F, F, F) {
    let t17687 = t4573 * t1506;
    let t17688 = t17687 * t3119;
    let t17689 = t4374 * t17688;
    let t17696 = -t15225 / F::new(54.0) + t15228 / F::new(288.0) - F::cast_from(0.1420012659563261767e0_f64) * t3116 * t17663 + t1111 * t17667 / F::new(48.0) + F::cast_from(0.71000632978163088351e-1_f64) * t3116 * t17670 + F::cast_from(0.94667510637550784468e-1_f64) * t15255 - t1111 * t17674 / F::new(48.0) + t1111 * t17677 / F::new(72.0) + F::cast_from(0.35973654042269298099e1_f64) * t15355 * t1509 + F::cast_from(0.18352229811776266582e0_f64) * t15327 * t1520 - F::cast_from(0.91572784804598301689e1_f64) * t15272 - F::cast_from(0.75734008510040627576e0_f64) * t12026 * t5325 + F::cast_from(0.71000632978163088351e-1_f64) * t3116 * t17689 - F::cast_from(0.56800506382530470682e0_f64) * t4363 * t5314 + F::cast_from(0.57954409931925052365e-1_f64) * t4369 * t5337;
    (t17688, t17689, t17696)
}
