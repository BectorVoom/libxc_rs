//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1201/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1201<F: Float>(t35631: F, t35643: F, t35646: F, t35662: F, t35664: F, t31464: F, t31468: F, t31475: F, t31477: F, t31479: F, t32823: F, t32824: F, t35629: F, t35635: F, t35638: F, t35651: F, t35656: F, t35660: F) -> F {
    let t37639 = F::cast_from(0.18868855373762491241e-2_f64) * t35631;
    let t37645 = F::new(13.0) / F::new(48.0) * t35643;
    let t37646 = F::new(0.305625e-1) * t35646;
    let t37652 = F::cast_from(0.45017719023973223821e-1_f64) * t35662;
    let t37653 = F::cast_from(0.22675591804667994221e-1_f64) * t35664;
    let t37654 = F::cast_from(0.62896184579208304138e-3_f64) * t35629 - t37639 + F::cast_from(0.31448092289604152068e-2_f64) * t35635 - F::cast_from(0.31448092289604152068e-2_f64) * t35638 - F::cast_from(0.41930789719472202758e-3_f64) * t31464 - F::cast_from(0.25158473831683321655e-2_f64) * t31468 - t32823 + t32824 - t31475 / F::new(96.0) + t37645 - t37646 - F::cast_from(0.26147916666666666666e0_f64) * t31477 + F::cast_from(0.75475421495049964966e-2_f64) * t35651 + F::cast_from(0.26416397523267487738e-1_f64) * t31479 + F::new(0.13753125e0) * t35656 + F::new(0.183375e0) * t35660 - t37652 - t37653;
    t37654
}
