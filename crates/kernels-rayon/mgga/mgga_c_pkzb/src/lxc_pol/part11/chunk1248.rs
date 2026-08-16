//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1248/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1248(t10859: f64, t1940: f64, t10833: f64, t10860: f64, t10863: f64, t17475: f64, t17478: f64, t17601: f64, t1916: f64, t1938: f64, t21203: f64, t2815: f64, t30620: f64, t30622: f64, t30624: f64, t30626: f64, t30628: f64, t30637: f64, t3581: f64, t5830: f64, t702: f64, t721: f64, t7324: f64, t7486: f64, t9419: f64, t9423: f64, t9426: f64, t9430: f64) -> f64 {
    let t30659 = t10859 * t1940;
    let t30663 = -t30620 - t30622 - t30624 + t30626 + t30628 + t30637 + 0.91082604192152556044e5_f64 * t17475 * t10833 * t17478 * t721 - 6.0_f64 * t7486 * t9419 + 0.96491876992155210402e2_f64 * t7324 * t9423 + 0.1929837539843104208e3_f64 * t7324 * t9426 + 0.62071215503128080361e4_f64 * t21203 * t9430 - 0.57895126195293126243e3_f64 * t5830 * t3581 * t2815 - 0.24828486201251232145e5_f64 * t17601 * t10863 * t702 - 2.0_f64 * t1916 * t10860 * t702 + 0.32163958997385070134e2_f64 * t1938 * t30659 * t702;
    t30663
}
