//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1321/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1321<F: Float>(t17630: F, t1950: F, t21134: F, t21173: F, t25861: F, t25863: F, t25865: F, t25867: F, t25869: F, t25872: F, t25876: F, t25878: F, t25880: F, t25883: F, t25885: F, t25887: F, t25889: F, t26023: F, t3578: F, t3605: F, t3608: F, t5820: F, t5877: F, t702: F, t714: F, t722: F, t9465: F) -> (F,) {
    let t26157 = -t25861 + t25863 + t25865 + t25867 - t25869 + t25872 - t25876 - t25878 - t25880 + t25883 + t25885 - t25887 - t25889 + 0.8276162067083744048e4 * t21173 * t21134 * t702 + 0.5848223622634646207e0 * t5877 * t3605 + 0.11696447245269292414e1 * t1950 * t9465 + 0.5848223622634646207e0 * t714 * t26023 * t722 + 0.17315859105681463759e2 * t17630 * t3608 + 1.0 * t5820 * t3578;
    (t26157,)
}
