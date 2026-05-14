//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1289/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1289<F: Float>(t21787: F, t2899: F, t9263: F, t2096: F, t9616: F, t17953: F, t17957: F, t18033: F, t18036: F, t18039: F, t2038: F, t2039: F, t2104: F, t2107: F, t21789: F, t21794: F, t25391: F, t300: F, t5729: F, t5956: F, t758: F, t7672: F, t7681: F, t7695: F, t7700: F, t7736: F, t7742: F, t9594: F) -> (F,) {
    let t25518 = t2899 * t21787 * t9263;
    let t25530 = t2096 * t9616;
    let t25548 = -0.22866142996303859718e-2 * t25518 - 0.22866142996303859718e-2 * t21789 + 0.11433071498151929859e-2 * t21794 - 0.51448821741683684367e-2 * t7736 * t7700 * t5956 * t7681 + 0.51448821741683684367e-2 * t7742 * t7700 * t5729 * t7681 - 0.15244095330869239812e-2 * t25530 + 0.5081365110289746604e-3 * t17953 + 0.1270341277572436651e-3 * t17957 - 0.42874018118069736972e-3 * t2038 * t758 * t25391 * t2039 - 0.47637797908966374413e-4 * t18033 - 0.95275595817932748826e-4 * t18036 + 0.47637797908966374413e-4 * t18039 + 0.51448821741683684368e-2 * t2104 * t7695 * t7672 - 0.10289764348336736874e-1 * t2104 * t300 * t9594 * t2107;
    (t25548,)
}
