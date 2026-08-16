//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1069/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1069(t2211: f64, t27059: f64, t35625: f64, t35629: f64, t35633: f64, t37786: f64, t37787: f64, t37788: f64, t37789: f64, t37790: f64, t40172: f64, t40177: f64, t40182: f64, t40185: f64, t40188: f64, t40191: f64, t40194: f64, t40198: f64, t40201: f64, t739: f64) -> f64 {
    let t43318 = 0.11974241701863808564e0_f64 * t739 * t2211 * t27059 - 0.5107751987195740728e-4_f64 * t40172 + 0.5107751987195740728e-4_f64 * t40177 + 0.1702583995731913576e-4_f64 * t40182 - 0.2727466165424534173e0_f64 * t40185 + 0.8182398496273602519e0_f64 * t40188 + 0.16364796992547205038e0_f64 * t40191 + 0.40911992481368012596e-1_f64 * t40194 - 0.86737941314158990616e-4_f64 * t40198 + 0.162600798888400151e-2_f64 * t40201 + t37786 - t37787 + t37788 - t37789 + t37790 + 0.1440846329149835838e-2_f64 * t35625 + 0.12195059916630011325e-2_f64 * t35629 + 0.12195059916630011325e-2_f64 * t35633;
    t43318
}
