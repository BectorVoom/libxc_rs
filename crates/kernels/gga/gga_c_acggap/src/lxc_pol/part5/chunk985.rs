//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 985/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk985<F: Float>(t5517: F, t868: F, t12282: F, t1265: F, t15221: F, t15223: F, t19607: F, t19611: F, t19615: F, t19618: F, t19620: F, t19627: F, t5520: F, t1308: F, t5368: F, t1614: F, t4131: F) -> (F, F, F) {
    let t19629 = t868 * t5517;
    let t19631 = -0.26341796731742046394e1 * t19607 + 0.26341796731742046394e1 * t19611 + 0.13170898365871023197e1 * t19615 + 0.13170898365871023197e1 * t19618 - 0.26341796731742046394e1 * t19620 + 0.13170898365871023197e1 * t12282 - 0.79025390195226139182e1 * t15221 - 0.13170898365871023197e1 * t15223 - 0.65854491829355115987e0 * t5520 * t1265 - 0.26341796731742046394e1 * t19627 - 0.26341796731742046394e1 * t19629;
    let t19637 = t1308 * t5368;
    let t19645 = t4131 * t1614;
    (t19631, t19637, t19645)
}
