//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1079/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1079<F: Float>(t1719: F, t2670: F, t16421: F, t183: F, t1034: F, t164: F, t167: F, t1717: F, t1721: F, t1753: F, t1783: F, t19949: F, t19961: F, t20067: F, t20071: F, t20102: F, t20137: F, t20252: F, t20419: F, t20427: F, t20441: F, t2639: F, t2682: F, t2693: F, t5407: F, t588: F, t600: F, t621: F, t6860: F, t6865: F, t6869: F, t7084: F, t7096: F, t7123: F, t7126: F, t7143: F) -> (F,) {
    let t20529 = t2670 * t1719;
    let t20542 = t16421 * t183;
    let t20553 = 0.39512695097613069591e1 * t2682 * t19949 - 0.19756347548806534796e1 * t7143 * t6860 - 0.23707617058567841754e2 * t7123 * t20071 - 0.19756347548806534796e1 * t588 * t7096 * t600 * t164 + 0.39512695097613069591e1 * t2682 * t19961 - 0.19756347548806534796e1 * t2693 * t20137 - 0.19756347548806534796e1 * t2693 * t20102 - 0.19756347548806534796e1 * t588 * t1783 * t2639 * t164 - 0.19756347548806534796e1 * t588 * t621 * t7084 * t164 - 0.11853808529283920877e2 * t7123 * t20427 - 0.65854491829355115987e0 * t588 * t5407 * t1034 * t164 + 0.39512695097613069591e1 * t1717 * t20529 * t1721 + 0.65854491829355115987e0 * t167 * t20441 - 0.65854491829355115987e0 * t2693 * t20252 - 0.19756347548806534796e1 * t588 * t20529 * t164 + 0.79025390195226139182e1 * t7126 * t6865 + 0.15805078039045227836e2 * t20542 * t20067 + 0.39512695097613069591e1 * t7126 * t6869 + 0.11853808529283920878e2 * t2682 * t20419 - 0.19756347548806534796e1 * t588 * t2670 * t1753 * t164;
    (t20553,)
}
