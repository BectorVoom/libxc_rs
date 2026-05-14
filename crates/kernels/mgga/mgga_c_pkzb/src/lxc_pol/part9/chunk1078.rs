//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1078/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1078<F: Float>(t19957: F, t19991: F, t20032: F, t20106: F, t20163: F, t20223: F, t20404: F, t20438: F, t5389: F, t621: F, t1044: F, t5373: F, t164: F, t1717: F, t1721: F, t1783: F, t183: F, t19953: F, t19966: F, t20019: F, t20081: F, t20093: F, t20114: F, t20195: F, t20398: F, t2594: F, t2647: F, t2682: F, t2693: F, t5240: F, t5251: F, t5367: F, t5391: F, t588: F, t6881: F, t6898: F, t6903: F, t6980: F, t7123: F, t7126: F, t7143: F, t9056: F, t9067: F) -> (F, F) {
    let t20441 = t19957 + t19991 + t20032 + t20106 + t20163 + t20223 + t20404 + t20438;
    let t20474 = t5389 * t621;
    let t20482 = t1044 * t5373;
    let t20498 = -0.11853808529283920877e2 * t7123 * t20195 - 0.19756347548806534796e1 * t9067 * t5240 - 0.65854491829355115987e0 * t588 * t183 * t20398 * t164 + 0.39512695097613069591e1 * t1717 * t1783 * t2594 + 0.13170898365871023197e1 * t2682 * t19953 + 0.11853808529283920877e2 * t7126 * t6903 - 0.39512695097613069591e1 * t7143 * t6980 - 0.65854491829355115987e0 * t2693 * t20093 - 0.19756347548806534796e1 * t7143 * t6881 - 0.19756347548806534796e1 * t2693 * t20081 - 0.19756347548806534796e1 * t2693 * t19966 + 0.11853808529283920877e2 * t2682 * t20019 - 0.11853808529283920877e2 * t20474 * t6898 + 0.92196288561097162379e1 * t2682 * t20114 - 0.19756347548806534796e1 * t588 * t1783 * t2647 - 0.39512695097613069591e1 * t5389 * t20482 * t5391 - 0.65854491829355115987e0 * t588 * t1044 * t5367 * t164 + 0.39512695097613069591e1 * t1717 * t20482 * t1721 - 0.65854491829355115987e0 * t588 * t20482 * t164 + 0.39512695097613069591e1 * t9056 * t5251;
    (t20441, t20498)
}
