//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1104/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1104<F: Float>(t1979: F, t7474: F, t2848: F, t5493: F, t5581: F, t7483: F, t1107: F, t1108: F, t17475: F, t17478: F, t17633: F, t1917: F, t1938: F, t1955: F, t1956: F, t1971: F, t1977: F, t2816: F, t2819: F, t2849: F, t2852: F, t5484: F, t5576: F, t5831: F, t5835: F, t5845: F, t5871: F, t721: F, t7296: F, t7299: F, t7300: F, t7303: F, t7309: F) -> (F, F) {
    let t20975 = t7474 * t1979;
    let t20982 = t2848 * t5493;
    let t21001 = 6.0 * t7483 * t5581;
    let t21002 = 18.0 * t1938 * t2816 * t1917 + 0.11579025239058625248e4 * t5871 * t2819 * t5831 + 0.10526802520742363173e2 * t5835 * t7296 + 0.10526802520742363173e2 * t1977 * t2849 * t1956 + 0.6233709278045326953e3 * t5845 * t2852 * t5484 + 0.10389515463408878255e3 * t5835 * t7300 + 0.51947577317044391277e2 * t1977 * t20975 * t721 + 0.51947577317044391277e2 * t1977 * t7299 * t1971 + 0.30762056574649219973e4 * t5845 * t20982 * t1956 + 0.51947577317044391277e2 * t5835 * t7303 + 0.17315859105681463759e2 * t1977 * t2852 * t5576 + 0.30762056574649219973e4 * t17633 * t7309 + 0.91082604192152556044e5 * t17475 * t1107 * t17478 * t5484 - 0.11696447245269292414e1 * t1955 * t1108 * t5576 + t21001;
    (t21001, t21002)
}
