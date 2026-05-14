//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1093/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1093<F: Float>(t29022: F, t29091: F, t29234: F, t29289: F, t29340: F, t29384: F, t29423: F, t29475: F, t1054: F, t3487: F, t2670: F, t3410: F, t1034: F, t10577: F, t164: F, t24792: F, t2682: F, t2693: F, t28995: F, t29004: F, t29013: F, t29248: F, t29366: F, t29370: F, t29374: F, t29403: F, t29410: F, t29415: F, t29424: F, t29454: F, t588: F, t7123: F, t7126: F, t8910: F, t8920: F, t8954: F, t8958: F, t8972: F, t9019: F, t9056: F, t9067: F) -> (F, F, F, F) {
    let t29478 = t29022 + t29091 + t29234 + t29289 + t29340 + t29384 + t29423 + t29475;
    let t29514 = t1054 * t3487;
    let t29562 = t2670 * t3410;
    let t29574 = -0.11853808529283920877e2 * t7123 * t29403 - 0.19756347548806534796e1 * t588 * t9019 * t1034 * t164 + 0.39512695097613069591e1 * t2682 * t29370 + 0.79025390195226139182e1 * t9056 * t8920 + 0.11853808529283920877e2 * t9056 * t8958 - 0.11853808529283920877e2 * t7123 * t29424 + 0.11853808529283920877e2 * t2682 * t28995 - 0.23707617058567841754e2 * t7123 * t29013 - 0.19756347548806534796e1 * t2693 * t29410 + 0.92196288561097162379e1 * t2682 * t29004 - 0.19756347548806534796e1 * t9067 * t8910 - 0.65854491829355115987e0 * t2693 * t29366 - 0.11853808529283920877e2 * t24792 * t8954 - 0.19756347548806534796e1 * t2693 * t29454 + 0.39512695097613069591e1 * t7126 * t10577 - 0.19756347548806534796e1 * t588 * t29562 * t164 - 0.19756347548806534796e1 * t2693 * t29415 + 0.39512695097613069591e1 * t2682 * t29374 - 0.65854491829355115987e0 * t2693 * t29248 - 0.19756347548806534796e1 * t9067 * t8972;
    (t29478, t29514, t29562, t29574)
}
