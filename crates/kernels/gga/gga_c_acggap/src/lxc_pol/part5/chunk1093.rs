//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1093/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1093<F: Float>(t1938: F, t3896: F, t6569: F, t857: F, t1907: F, t323: F, t851: F, t1308: F, t5379: F, t1620: F, t4137: F, t1215: F, t1220: F, t12250: F, t12257: F, t12259: F, t12263: F, t12268: F, t12271: F, t12276: F, t15184: F, t15190: F, t15192: F, t15196: F, t15199: F, t446: F, t463: F, t6438: F, t6557: F) -> F {
    let t19567 = t3896 * t1938;
    let t19577 = t857 * t6569;
    let t19582 = t851 * t1907 * t323;
    let t19588 = t1308 * t5379;
    let t19593 = t4137 * t1620;
    let t19595 = -F::new(0.13170898365871023197e1) * t19567 - F::new(0.13170898365871023197e1) * t12250 - F::new(0.26341796731742046394e1) * t15184 + F::new(0.26341796731742046394e1) * t446 * t1220 * t6557 * t463 + t12257 + F::new(0.26341796731742046394e1) * t1215 * t6569 + F::new(0.26341796731742046394e1) * t19577 + F::new(0.79025390195226139182e1) * t12259 - F::new(0.13170898365871023197e1) * t12263 + t12268 - F::new(0.13170898365871023197e1) * t19582 - F::new(0.79025390195226139182e1) * t1215 * t6438 + F::new(0.10536718692696818558e2) * t15190 + F::new(0.52683593463484092788e1) * t15192 + F::new(0.26341796731742046394e1) * t19588 + F::new(0.65854491829355115987e0) * t12271 - t12276 + F::new(0.26341796731742046394e1) * t15196 - F::new(0.26341796731742046394e1) * t15199 + F::new(0.52683593463484092788e1) * t19593;
    t19595
}
