//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1201/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1201(t35631: f64, t35643: f64, t35646: f64, t35662: f64, t35664: f64, t31464: f64, t31468: f64, t31475: f64, t31477: f64, t31479: f64, t32823: f64, t32824: f64, t35629: f64, t35635: f64, t35638: f64, t35651: f64, t35656: f64, t35660: f64) -> f64 {
    let t37639 = 0.18868855373762491241e-2_f64 * t35631;
    let t37645 = 13.0_f64 / 48.0_f64 * t35643;
    let t37646 = 0.305625e-1_f64 * t35646;
    let t37652 = 0.45017719023973223821e-1_f64 * t35662;
    let t37653 = 0.22675591804667994221e-1_f64 * t35664;
    let t37654 = 0.62896184579208304138e-3_f64 * t35629 - t37639 + 0.31448092289604152068e-2_f64 * t35635 - 0.31448092289604152068e-2_f64 * t35638 - 0.41930789719472202758e-3_f64 * t31464 - 0.25158473831683321655e-2_f64 * t31468 - t32823 + t32824 - t31475 / 96.0_f64 + t37645 - t37646 - 0.26147916666666666666e0_f64 * t31477 + 0.75475421495049964966e-2_f64 * t35651 + 0.26416397523267487738e-1_f64 * t31479 + 0.13753125e0_f64 * t35656 + 0.183375e0_f64 * t35660 - t37652 - t37653;
    t37654
}
