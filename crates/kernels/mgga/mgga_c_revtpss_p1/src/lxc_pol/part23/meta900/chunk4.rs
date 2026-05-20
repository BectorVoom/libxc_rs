//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2865/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2865<F: Float>(t221: F, t23245: F, t2484: F, t2485: F, t23168: F, t40352: F, t62429: F, t62431: F, t62435: F, t62439: F, t62441: F, t62443: F, t62445: F, t62453: F, t62458: F, t62460: F, t62475: F, t62494: F, t62498: F, t62502: F, t76887: F, t77120: F, t825: F, t827: F, t828: F) -> F {
    let t77127 = t2484 * t2485 * t221 * t23245;
    let t77131 = t40352 * t2485 * t221 * t23168;
    let t77147 = -F::cast_from(0.12705000702321332056e-4_f64) * t76887 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t77120 - F::cast_from(0.12705000702321332056e-4_f64) * t77127 - F::cast_from(0.7623000421392799234e-4_f64) * t77131 - F::cast_from(0.42874018118069736972e-3_f64) * t62429 - F::cast_from(0.81312004494856525161e-2_f64) * t62431 + F::cast_from(0.25724410870841842183e-2_f64) * t62435 - F::cast_from(0.85748036236139473944e-3_f64) * t62439 + F::cast_from(0.12004725073059526352e-1_f64) * t62441 + F::cast_from(0.45732285992607719437e-3_f64) * t62443 - F::cast_from(0.22866142996303859719e-3_f64) * t62445 - F::cast_from(0.15246000842785598468e-3_f64) * t62453 + F::cast_from(0.21437009059034868486e-4_f64) * t62458 + F::cast_from(0.48018900292238105408e-1_f64) * t62460 - F::cast_from(0.24009450146119052704e-1_f64) * t62475 - F::cast_from(0.12004725073059526352e-1_f64) * t62494 + F::cast_from(0.30492001685571196935e-3_f64) * t62498 + F::cast_from(0.30492001685571196935e-3_f64) * t62502;
    t77147
}
