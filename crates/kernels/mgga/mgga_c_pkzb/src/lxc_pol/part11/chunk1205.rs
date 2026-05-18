//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1205/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1205<F: Float>(t10627: F, t621: F, t1044: F, t10573: F, t10621: F, t10675: F, t164: F, t167: F, t1717: F, t1721: F, t183: F, t20542: F, t2594: F, t2639: F, t2647: F, t2670: F, t2682: F, t2693: F, t29018: F, t29024: F, t29210: F, t29361: F, t29399: F, t29478: F, t29562: F, t3441: F, t3460: F, t5389: F, t5391: F, t588: F, t600: F, t7143: F, t8888: F, t8949: F, t8967: F, t9056: F, t9067: F) -> F {
    let t29613 = t621 * t10627;
    let t29634 = -F::new(0.65854491829355115987e0) * t588 * t10675 * t600 * t164 - F::new(0.19756347548806534796e1) * t2693 * t29361 - F::new(0.65854491829355115987e0) * t588 * t183 * t29210 * t164 - F::new(0.19756347548806534796e1) * t588 * t3460 * t2647 - F::new(0.19756347548806534796e1) * t588 * t1044 * t8888 * t164 + F::new(0.11853808529283920877e2) * t2682 * t29399 - F::new(0.19756347548806534796e1) * t7143 * t10573 - F::new(0.19756347548806534796e1) * t588 * t2670 * t3441 * t164 + F::new(0.65854491829355115987e0) * t167 * t29478 - F::new(0.65854491829355115987e0) * t588 * t621 * t10621 * t164 + F::new(0.39512695097613069592e1) * t9056 * t8949 + F::new(0.39512695097613069592e1) * t1717 * t3460 * t2594 + F::new(0.15805078039045227836e2) * t20542 * t29024 - F::new(0.39512695097613069591e1) * t5389 * t29613 * t5391 - F::new(0.39512695097613069592e1) * t9067 * t8967 + F::new(0.39512695097613069591e1) * t1717 * t29613 * t1721 - F::new(0.19756347548806534796e1) * t588 * t3460 * t2639 * t164 + F::new(0.39512695097613069591e1) * t1717 * t29562 * t1721 - F::new(0.65854491829355115987e0) * t588 * t29613 * t164 + F::new(0.13170898365871023197e1) * t2682 * t29018;
    t29634
}
