//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2865/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2865(t221: f64, t23245: f64, t2484: f64, t2485: f64, t23168: f64, t40352: f64, t62429: f64, t62431: f64, t62435: f64, t62439: f64, t62441: f64, t62443: f64, t62445: f64, t62453: f64, t62458: f64, t62460: f64, t62475: f64, t62494: f64, t62498: f64, t62502: f64, t76887: f64, t77120: f64, t825: f64, t827: f64, t828: f64) -> f64 {
    let t77127 = t2484 * t2485 * t221 * t23245;
    let t77131 = t40352 * t2485 * t221 * t23168;
    let t77147 = -0.12705000702321332056e-4_f64 * t76887 - 0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t77120 - 0.12705000702321332056e-4_f64 * t77127 - 0.7623000421392799234e-4_f64 * t77131 - 0.42874018118069736972e-3_f64 * t62429 - 0.81312004494856525161e-2_f64 * t62431 + 0.25724410870841842183e-2_f64 * t62435 - 0.85748036236139473944e-3_f64 * t62439 + 0.12004725073059526352e-1_f64 * t62441 + 0.45732285992607719437e-3_f64 * t62443 - 0.22866142996303859719e-3_f64 * t62445 - 0.15246000842785598468e-3_f64 * t62453 + 0.21437009059034868486e-4_f64 * t62458 + 0.48018900292238105408e-1_f64 * t62460 - 0.24009450146119052704e-1_f64 * t62475 - 0.12004725073059526352e-1_f64 * t62494 + 0.30492001685571196935e-3_f64 * t62498 + 0.30492001685571196935e-3_f64 * t62502;
    t77147
}
