//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 884/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk884<F: Float>(t588: F, t621: F, t1044: F, t1719: F, t1034: F, t164: F, t167: F, t1717: F, t1721: F, t1753: F, t1783: F, t183: F, t2594: F, t2639: F, t2647: F, t2670: F, t2682: F, t2693: F, t600: F, t6860: F, t6865: F, t6869: F, t6881: F, t6898: F, t6903: F, t6980: F, t7084: F, t7096: F, t7123: F, t7126: F) -> (F, F) {
    let t7143 = t588 * t621;
    let t7156 = t1044 * t1719;
    let t7173 = -0.39512695097613069591e1 * t7123 * t6898 + 0.26341796731742046394e1 * t7126 * t2594 + 0.26341796731742046394e1 * t2682 * t6865 + 0.39512695097613069591e1 * t2682 * t6903 + 0.13170898365871023197e1 * t2682 * t6869 - 0.65854491829355115987e0 * t588 * t1783 * t1034 * t164 - 0.13170898365871023197e1 * t588 * t621 * t2639 * t164 - 0.13170898365871023197e1 * t7143 * t2647 - 0.65854491829355115987e0 * t588 * t183 * t7084 * t164 - 0.13170898365871023197e1 * t2693 * t6980 - 0.65854491829355115987e0 * t2693 * t6860 - 0.65854491829355115987e0 * t2693 * t6881 + 0.13170898365871023197e1 * t1717 * t7156 * t1721 - 0.13170898365871023197e1 * t588 * t2670 * t600 * t164 - 0.65854491829355115987e0 * t588 * t1044 * t1753 * t164 - 0.65854491829355115987e0 * t588 * t7156 * t164 + 0.65854491829355115987e0 * t167 * t7096;
    (t7143, t7173)
}
