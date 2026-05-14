//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1323/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1323<F: Float>(t3604: F, t5493: F, t1916: F, t1955: F, t1956: F, t1971: F, t1977: F, t25921: F, t25924: F, t25927: F, t25930: F, t25933: F, t25936: F, t25939: F, t25943: F, t25946: F, t25949: F, t25953: F, t26134: F, t2852: F, t3605: F, t3608: F, t5838: F, t5845: F, t703: F, t7474: F, t9451: F) -> (F,) {
    let t26211 = t3604 * t5493;
    let t26218 = -t25921 + t25924 + t25927 + t25930 + t25933 + t25936 - t25939 - t25943 - t25946 - t25949 - t25953 - 4.0 * t1916 * t26134 * t703 - 0.10389515463408878255e3 * t5838 * t3608 * t1971 - 0.11696447245269292414e1 * t1955 * t3605 * t1971 - 0.10389515463408878255e3 * t5838 * t9451 * t1956 + 0.17315859105681463759e2 * t1977 * t9451 * t1971 + 0.10254018858216406658e4 * t5845 * t26211 * t1956 + 0.34631718211362927518e2 * t1977 * t2852 * t7474;
    (t26218,)
}
