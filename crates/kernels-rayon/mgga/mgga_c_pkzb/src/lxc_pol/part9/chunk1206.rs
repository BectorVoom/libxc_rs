//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1206/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1206(t1979: f64, t7474: f64, t2848: f64, t5493: f64, t5581: f64, t7483: f64, t1107: f64, t1108: f64, t17475: f64, t17478: f64, t17633: f64, t1917: f64, t1938: f64, t1955: f64, t1956: f64, t1971: f64, t1977: f64, t2816: f64, t2819: f64, t2849: f64, t2852: f64, t5484: f64, t5576: f64, t5831: f64, t5835: f64, t5845: f64, t5871: f64, t721: f64, t7296: f64, t7299: f64, t7300: f64, t7303: f64, t7309: f64) -> (f64, f64) {
    let t20975 = t7474 * t1979;
    let t20982 = t2848 * t5493;
    let t21001 = 6.0_f64 * t7483 * t5581;
    let t21002 = 18.0_f64 * t1938 * t2816 * t1917 + 0.11579025239058625248e4_f64 * t5871 * t2819 * t5831 + 0.10526802520742363173e2_f64 * t5835 * t7296 + 0.10526802520742363173e2_f64 * t1977 * t2849 * t1956 + 0.6233709278045326953e3_f64 * t5845 * t2852 * t5484 + 0.10389515463408878255e3_f64 * t5835 * t7300 + 0.51947577317044391277e2_f64 * t1977 * t20975 * t721 + 0.51947577317044391277e2_f64 * t1977 * t7299 * t1971 + 0.30762056574649219973e4_f64 * t5845 * t20982 * t1956 + 0.51947577317044391277e2_f64 * t5835 * t7303 + 0.17315859105681463759e2_f64 * t1977 * t2852 * t5576 + 0.30762056574649219973e4_f64 * t17633 * t7309 + 0.91082604192152556044e5_f64 * t17475 * t1107 * t17478 * t5484 - 0.11696447245269292414e1_f64 * t1955 * t1108 * t5576 + t21001;
    (t21001, t21002)
}
