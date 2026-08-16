//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1127/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1127(t406: f64, t6263: f64, t394: f64, t6413: f64, t1160: f64, t1539: f64, t19748: f64, t377: f64, t6523: f64, t1934: f64, t848: f64, t1170: f64, t12385: f64, t1411: f64, t151: f64, t1530: f64, t1629: f64, t19038: f64, t19040: f64, t19042: f64, t19045: f64, t19048: f64, t19053: f64, t19060: f64, t407: f64, t4166: f64) -> (f64, f64) {
    let t20138 = t6263 * t406;
    let t20142 = t394 * t6413;
    let t20149 = t1160 * t19748 * t1539;
    let t20157 = t377 * t6523;
    let t20159 = t848 * t1934;
    let t20161 = 0.52683593463484092788e1_f64 * t19038 - 0.13170898365871023197e1_f64 * t19040 - 0.26341796731742046394e1_f64 * t19042 + 0.15805078039045227836e2_f64 * t1530 * t1629 * t20138 - 0.13170898365871023197e1_f64 * t151 * t20142 * t407 - 0.13170898365871023197e1_f64 * t19045 - 0.52683593463484092788e1_f64 * t19048 + 0.13170898365871023197e1_f64 * t20149 - 0.26341796731742046394e1_f64 * t19053 - 0.26341796731742046394e1_f64 * t1170 * t4166 * t1411 - 0.65854491829355115987e0_f64 * t12385 + 0.39512695097613069591e1_f64 * t19060 - 0.26341796731742046394e1_f64 * t20157 + 0.65854491829355115987e0_f64 * t20159;
    (t20138, t20161)
}
