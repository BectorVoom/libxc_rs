//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1156/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1156(t7447: f64, t8823: f64, t7440: f64, t8826: f64, t1488: f64, t2030: f64, t2031: f64, t507: f64, t7807: f64, t31605: f64, t35812: f64, t35814: f64, t35817: f64, t35818: f64, t35823: f64, t35828: f64, t35829: f64, t35833: f64, t35836: f64, t35838: f64, t35841: f64, t35845: f64, t35846: f64) -> f64 {
    let t35848 = t7447 * t8823;
    let t35849 = 0.84046875e-1_f64 * t35848;
    let t35850 = t7440 * t8826;
    let t35851 = 0.5603125e-1_f64 * t35850;
    let t35853 = t2030 * t1488 * t2031;
    let t35856 = t2030 * t507 * t7807;
    let t35858 = t35812 + 0.38110238327173099531e-2_f64 * t31605 + 0.80031500487063509014e-2_f64 * t35814 + t35817 + 0.14291339372689912324e-3_f64 * t35818 + 0.10718504529517434243e-3_f64 * t35823 + t35828 - 0.80031500487063509014e-2_f64 * t35829 + 0.18868855373762491241e-2_f64 * t35833 - t35836 + t35838 + 0.53592522647587171215e-3_f64 * t35841 + t35845 - 0.17149607247227894789e-1_f64 * t35846 - t35849 - t35851 + 0.22921875e-1_f64 * t35853 + 0.114609375e-1_f64 * t35856;
    t35858
}
