//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 803/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk803<F: Float>(t1044: F, t588: F, t1034: F, t164: F, t167: F, t1717: F, t1721: F, t183: F, t2594: F, t2639: F, t2647: F, t2670: F, t2682: F, t2693: F, t3441: F, t3460: F, t600: F, t621: F, t7123: F, t8888: F, t8910: F, t8920: F, t8949: F, t8954: F, t8958: F, t8967: F, t8972: F, t9019: F, t9048: F, t9056: F) -> (F, F) {
    let t9067 = t588 * t1044;
    let t9095 = -0.39512695097613069591e1 * t7123 * t8954 + 0.13170898365871023197e1 * t1717 * t9048 * t1721 + 0.26341796731742046394e1 * t2682 * t8920 + 0.39512695097613069591e1 * t2682 * t8958 + 0.26341796731742046394e1 * t9056 * t2594 - 0.13170898365871023197e1 * t588 * t2670 * t1034 * t164 - 0.13170898365871023197e1 * t588 * t1044 * t2639 * t164 - 0.13170898365871023197e1 * t9067 * t2647 + 0.13170898365871023197e1 * t2682 * t8949 - 0.65854491829355115987e0 * t588 * t621 * t3441 * t164 - 0.65854491829355115987e0 * t588 * t183 * t8888 * t164 - 0.65854491829355115987e0 * t2693 * t8910 - 0.65854491829355115987e0 * t588 * t9048 * t164 - 0.13170898365871023197e1 * t2693 * t8967 - 0.65854491829355115987e0 * t2693 * t8972 - 0.65854491829355115987e0 * t588 * t3460 * t600 * t164 + 0.65854491829355115987e0 * t167 * t9019;
    (t9067, t9095)
}
