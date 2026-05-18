//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1311/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1311<F: Float>(t13584: F, t20944: F, t6465: F, t3073: F, t4241: F, t1160: F, t1629: F, t23688: F, t1170: F, t14570: F, t19216: F, t19222: F, t19224: F, t19235: F, t19237: F, t19240: F, t19243: F, t19246: F, t19718: F, t407: F, t6482: F, t930: F) -> F {
    let t24419 = t13584 * t6465 * t20944;
    let t24422 = t3073 * t6465 * t4241;
    let t24426 = t1160 * t1629 * t23688;
    let t24441 = F::new(0.13170898365871023197e1) * t19216 + F::new(0.79025390195226139182e1) * t24419 - F::new(0.79025390195226139182e1) * t24422 + F::new(0.79025390195226139182e1) * t19222 + F::new(0.26341796731742046394e1) * t24426 - F::new(0.39512695097613069591e1) * t19224 - F::new(0.65854491829355115987e0) * t1170 * t6482 * t930 - F::new(0.65854491829355115987e0) * t14570 - F::new(0.52683593463484092788e1) * t19235 - F::new(0.26341796731742046394e1) * t19237 + F::new(0.39512695097613069591e1) * t19240 - F::new(0.13170898365871023197e1) * t1170 * t19718 * t407 + F::new(0.26341796731742046394e1) * t19243 + F::new(0.26341796731742046394e1) * t19246;
    t24441
}
