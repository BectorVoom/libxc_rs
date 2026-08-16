//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1093/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1093(t1938: f64, t3896: f64, t6569: f64, t857: f64, t1907: f64, t323: f64, t851: f64, t1308: f64, t5379: f64, t1620: f64, t4137: f64, t1215: f64, t1220: f64, t12250: f64, t12257: f64, t12259: f64, t12263: f64, t12268: f64, t12271: f64, t12276: f64, t15184: f64, t15190: f64, t15192: f64, t15196: f64, t15199: f64, t446: f64, t463: f64, t6438: f64, t6557: f64) -> f64 {
    let t19567 = t3896 * t1938;
    let t19577 = t857 * t6569;
    let t19582 = t851 * t1907 * t323;
    let t19588 = t1308 * t5379;
    let t19593 = t4137 * t1620;
    let t19595 = -0.13170898365871023197e1_f64 * t19567 - 0.13170898365871023197e1_f64 * t12250 - 0.26341796731742046394e1_f64 * t15184 + 0.26341796731742046394e1_f64 * t446 * t1220 * t6557 * t463 + t12257 + 0.26341796731742046394e1_f64 * t1215 * t6569 + 0.26341796731742046394e1_f64 * t19577 + 0.79025390195226139182e1_f64 * t12259 - 0.13170898365871023197e1_f64 * t12263 + t12268 - 0.13170898365871023197e1_f64 * t19582 - 0.79025390195226139182e1_f64 * t1215 * t6438 + 0.10536718692696818558e2_f64 * t15190 + 0.52683593463484092788e1_f64 * t15192 + 0.26341796731742046394e1_f64 * t19588 + 0.65854491829355115987e0_f64 * t12271 - t12276 + 0.26341796731742046394e1_f64 * t15196 - 0.26341796731742046394e1_f64 * t15199 + 0.52683593463484092788e1_f64 * t19593;
    t19595
}
