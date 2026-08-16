//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1177/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1177<F: Float>(t1044: F, t164: F, t1717: F, t1721: F, t1783: F, t183: F, t19953: F, t19966: F, t20019: F, t20081: F, t20093: F, t20114: F, t20195: F, t20398: F, t20474: F, t20482: F, t2594: F, t2647: F, t2682: F, t2693: F, t5240: F, t5251: F, t5367: F, t5389: F, t5391: F, t588: F, t6881: F, t6898: F, t6903: F, t6980: F, t7123: F, t7126: F, t7143: F, t9056: F, t9067: F) -> F {
    let t20498 = -F::cast_from(0.11853808529283920877e2_f64) * t7123 * t20195 - F::cast_from(0.19756347548806534796e1_f64) * t9067 * t5240 - F::cast_from(0.65854491829355115987e0_f64) * t588 * t183 * t20398 * t164 + F::cast_from(0.39512695097613069591e1_f64) * t1717 * t1783 * t2594 + F::cast_from(0.13170898365871023197e1_f64) * t2682 * t19953 + F::cast_from(0.11853808529283920877e2_f64) * t7126 * t6903 - F::cast_from(0.39512695097613069591e1_f64) * t7143 * t6980 - F::cast_from(0.65854491829355115987e0_f64) * t2693 * t20093 - F::cast_from(0.19756347548806534796e1_f64) * t7143 * t6881 - F::cast_from(0.19756347548806534796e1_f64) * t2693 * t20081 - F::cast_from(0.19756347548806534796e1_f64) * t2693 * t19966 + F::cast_from(0.11853808529283920877e2_f64) * t2682 * t20019 - F::cast_from(0.11853808529283920877e2_f64) * t20474 * t6898 + F::cast_from(0.92196288561097162379e1_f64) * t2682 * t20114 - F::cast_from(0.19756347548806534796e1_f64) * t588 * t1783 * t2647 - F::cast_from(0.39512695097613069591e1_f64) * t5389 * t20482 * t5391 - F::cast_from(0.65854491829355115987e0_f64) * t588 * t1044 * t5367 * t164 + F::cast_from(0.39512695097613069591e1_f64) * t1717 * t20482 * t1721 - F::cast_from(0.65854491829355115987e0_f64) * t588 * t20482 * t164 + F::cast_from(0.39512695097613069591e1_f64) * t9056 * t5251;
    t20498
}
