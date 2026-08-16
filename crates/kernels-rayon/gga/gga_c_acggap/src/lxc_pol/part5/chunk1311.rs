//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1311/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1311(t13584: f64, t20944: f64, t6465: f64, t3073: f64, t4241: f64, t1160: f64, t1629: f64, t23688: f64, t1170: f64, t14570: f64, t19216: f64, t19222: f64, t19224: f64, t19235: f64, t19237: f64, t19240: f64, t19243: f64, t19246: f64, t19718: f64, t407: f64, t6482: f64, t930: f64) -> f64 {
    let t24419 = t13584 * t6465 * t20944;
    let t24422 = t3073 * t6465 * t4241;
    let t24426 = t1160 * t1629 * t23688;
    let t24441 = 0.13170898365871023197e1_f64 * t19216 + 0.79025390195226139182e1_f64 * t24419 - 0.79025390195226139182e1_f64 * t24422 + 0.79025390195226139182e1_f64 * t19222 + 0.26341796731742046394e1_f64 * t24426 - 0.39512695097613069591e1_f64 * t19224 - 0.65854491829355115987e0_f64 * t1170 * t6482 * t930 - 0.65854491829355115987e0_f64 * t14570 - 0.52683593463484092788e1_f64 * t19235 - 0.26341796731742046394e1_f64 * t19237 + 0.39512695097613069591e1_f64 * t19240 - 0.13170898365871023197e1_f64 * t1170 * t19718 * t407 + 0.26341796731742046394e1_f64 * t19243 + 0.26341796731742046394e1_f64 * t19246;
    t24441
}
