//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1248/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1248<F: Float>(t10859: F, t1940: F, t10833: F, t10860: F, t10863: F, t17475: F, t17478: F, t17601: F, t1916: F, t1938: F, t21203: F, t2815: F, t30620: F, t30622: F, t30624: F, t30626: F, t30628: F, t30637: F, t3581: F, t5830: F, t702: F, t721: F, t7324: F, t7486: F, t9419: F, t9423: F, t9426: F, t9430: F) -> F {
    let t30659 = t10859 * t1940;
    let t30663 = -t30620 - t30622 - t30624 + t30626 + t30628 + t30637 + F::cast_from(0.91082604192152556044e5_f64) * t17475 * t10833 * t17478 * t721 - F::new(6.0) * t7486 * t9419 + F::cast_from(0.96491876992155210402e2_f64) * t7324 * t9423 + F::cast_from(0.1929837539843104208e3_f64) * t7324 * t9426 + F::cast_from(0.62071215503128080361e4_f64) * t21203 * t9430 - F::cast_from(0.57895126195293126243e3_f64) * t5830 * t3581 * t2815 - F::cast_from(0.24828486201251232145e5_f64) * t17601 * t10863 * t702 - F::new(2.0) * t1916 * t10860 * t702 + F::cast_from(0.32163958997385070134e2_f64) * t1938 * t30659 * t702;
    t30663
}
