//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3554/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3554<F: Float>(t19462: F, t3286: F, t12154: F, t16393: F, t16468: F, t16506: F, t16544: F, t16559: F, t16560: F, t16574: F, t19450: F, t19453: F, t19484: F, t19492: F, t19549: F, t19557: F, t19569: F, t19602: F, t19607: F, t19608: F, t19612: F, t3133: F, t3223: F, t3288: F, t3317: F, t3318: F, t4964: F, t4984: F, t4999: F, t55575: F, t55632: F, t55934: F, t55944: F, t67618: F, t989: F) -> F {
    let t67714 = t19462 * t3286;
    let t67723 = -F::cast_from(0.65854491829355115987e0_f64) * t3317 * t67618 * t3318 + F::cast_from(0.79025390195226139182e1_f64) * t55632 * t19549 - F::cast_from(0.79025390195226139182e1_f64) * t55575 * t19492 + F::cast_from(0.52683593463484092788e1_f64) * t989 * t19602 * t4984 - F::cast_from(0.26341796731742046394e1_f64) * t989 * t19607 * t4999 + F::cast_from(0.13170898365871023197e1_f64) * t55944 * t19453 - F::cast_from(0.13170898365871023197e1_f64) * t19569 * t16574 - F::cast_from(0.39512695097613069591e1_f64) * t16559 * t19450 * t16560 * t3133 - F::cast_from(0.26341796731742046394e1_f64) * t55934 * t4964 - F::cast_from(0.13170898365871023197e1_f64) * t16544 * t16393 - F::cast_from(0.13170898365871023197e1_f64) * t12154 * t19612 - F::cast_from(0.26341796731742046394e1_f64) * t16506 * t19484 - F::cast_from(0.13170898365871023197e1_f64) * t67714 * t3288 - F::cast_from(0.13170898365871023197e1_f64) * t19608 * t16574 - F::cast_from(0.13170898365871023197e1_f64) * t3223 * t19557 - F::cast_from(0.26341796731742046394e1_f64) * t16544 * t16468;
    t67723
}
