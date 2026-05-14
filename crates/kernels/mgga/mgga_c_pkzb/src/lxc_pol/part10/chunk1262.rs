//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1262/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1262<F: Float>(t164: F, t1717: F, t1783: F, t24221: F, t24311: F, t24337: F, t24350: F, t24407: F, t24443: F, t24768: F, t2594: F, t2670: F, t2682: F, t2693: F, t3441: F, t588: F, t621: F, t7126: F, t7143: F, t8888: F, t8949: F, t8967: F, t8972: F) -> (F,) {
    let t24834 = -0.13170898365871023197e1 * t2693 * t24311 - 0.26341796731742046394e1 * t7143 * t8967 - 0.65854491829355115987e0 * t588 * t24768 * t164 - 0.13170898365871023197e1 * t7143 * t8972 + 0.39512695097613069591e1 * t2682 * t24407 - 0.65854491829355115987e0 * t2693 * t24337 + 0.13170898365871023197e1 * t2682 * t24221 - 0.65854491829355115987e0 * t588 * t1783 * t3441 * t164 + 0.26341796731742046394e1 * t2682 * t24350 + 0.26341796731742046394e1 * t7126 * t8949 - 0.13170898365871023197e1 * t588 * t621 * t8888 * t164 - 0.26341796731742046394e1 * t2693 * t24443 + 0.52683593463484092788e1 * t1717 * t2670 * t2594;
    (t24834,)
}
