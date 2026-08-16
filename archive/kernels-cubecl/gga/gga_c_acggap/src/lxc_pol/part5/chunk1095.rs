//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1095/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1095<F: Float>(t1659: F, t4137: F, t5517: F, t852: F, t868: F, t12282: F, t1265: F, t15221: F, t15223: F, t19607: F, t19611: F, t19615: F, t19618: F, t5520: F) -> F {
    let t19620 = t4137 * t1659;
    let t19627 = t852 * t5517;
    let t19629 = t868 * t5517;
    let t19631 = -F::cast_from(0.26341796731742046394e1_f64) * t19607 + F::cast_from(0.26341796731742046394e1_f64) * t19611 + F::cast_from(0.13170898365871023197e1_f64) * t19615 + F::cast_from(0.13170898365871023197e1_f64) * t19618 - F::cast_from(0.26341796731742046394e1_f64) * t19620 + F::cast_from(0.13170898365871023197e1_f64) * t12282 - F::cast_from(0.79025390195226139182e1_f64) * t15221 - F::cast_from(0.13170898365871023197e1_f64) * t15223 - F::cast_from(0.65854491829355115987e0_f64) * t5520 * t1265 - F::cast_from(0.26341796731742046394e1_f64) * t19627 - F::cast_from(0.26341796731742046394e1_f64) * t19629;
    t19631
}
