//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2190/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2190<F: Float>(t1307: F, t22635: F, t26331: F, t567: F, t6347: F, t1985: F, t20022: F, t6889: F, t6906: F, t28192: F, t80727: F, t1375: F, t1842: F, t20029: F, t26471: F, t26472: F, t26482: F, t3887: F, t5215: F, t5321: F, t6993: F, t91487: F, t97640: F, t97644: F, t97647: F) -> F {
    let t97652 = t26331 * t22635 * t567 * t6347 * t1307;
    let t97658 = t1985 * t6889 * t6906 * t20022;
    let t97664 = t80727 * t28192;
    let t97666 = -F::cast_from(2.0_f64) * t20029 * t6993 + F::cast_from(4.0_f64) * t5321 * t26482 + F::cast_from(0.16449340668482264365e-1_f64) * t97640 + F::cast_from(0.3289868133696452873e-1_f64) * t97644 + F::cast_from(0.3289868133696452873e-1_f64) * t97647 + F::cast_from(0.49348022005446793095e-1_f64) * t97652 - F::cast_from(2.0_f64) * t5215 * t26472 - F::cast_from(0.82246703342411321825e-2_f64) * t97658 + t91487 + F::cast_from(4.0_f64) * t1375 * t3887 * t26471 * t1842 - F::cast_from(0.11514538467937585055e0_f64) * t97664;
    t97666
}
