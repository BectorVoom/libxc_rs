//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1309/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1309(t1160: f64, t1539: f64, t19807: f64, t4210: f64, t6482: f64, t14518: f64, t14525: f64, t14528: f64, t14534: f64, t151: f64, t1530: f64, t1533: f64, t1629: f64, t19161: f64, t19172: f64, t19176: f64, t19179: f64, t19181: f64, t20228: f64, t24113: f64, t4198: f64, t6551: f64, t955: f64) -> f64 {
    let t24388 = t1160 * t19807 * t1539;
    let t24392 = t1160 * t6482 * t4210;
    let t24395 = -0.65854491829355115987e0_f64 * t151 * t6551 * t955 - 0.15805078039045227836e2_f64 * t4198 * t1629 * t24113 + 0.26341796731742046394e1_f64 * t1530 * t20228 * t1533 + 0.52683593463484092788e1_f64 * t19161 - 0.26341796731742046394e1_f64 * t14518 + 0.26341796731742046394e1_f64 * t19172 + 0.79025390195226139182e1_f64 * t14525 + 0.65854491829355115987e0_f64 * t14528 + 0.79025390195226139182e1_f64 * t19176 + 0.13170898365871023197e1_f64 * t19179 + 0.26341796731742046394e1_f64 * t24388 - 0.13170898365871023197e1_f64 * t19181 + 0.13170898365871023197e1_f64 * t24392 - 0.13170898365871023197e1_f64 * t14534;
    t24395
}
