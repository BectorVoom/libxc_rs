//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1264/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1264<F: Float>(t164: F, t20542: F, t24110: F, t24114: F, t24131: F, t24211: F, t24226: F, t24415: F, t24431: F, t2639: F, t2670: F, t2682: F, t2693: F, t588: F, t6860: F, t6869: F, t6980: F, t7123: F, t7126: F, t7143: F, t8910: F, t8920: F, t9056: F, t9067: F) -> (F,) {
    let t24898 = 0.15805078039045227836e2 * t20542 * t24110 + 0.26341796731742046394e1 * t2682 * t24131 - 0.26341796731742046394e1 * t9067 * t6980 - 0.26341796731742046394e1 * t588 * t2670 * t2639 * t164 - 0.15805078039045227836e2 * t7123 * t24431 - 0.13170898365871023197e1 * t7143 * t8910 - 0.65854491829355115987e0 * t2693 * t24211 + 0.92196288561097162379e1 * t2682 * t24415 - 0.13170898365871023197e1 * t9067 * t6860 + 0.26341796731742046394e1 * t9056 * t6869 + 0.52683593463484092788e1 * t7126 * t8920 - 0.65854491829355115987e0 * t2693 * t24226 - 0.23707617058567841754e2 * t7123 * t24114;
    (t24898,)
}
