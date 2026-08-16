//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1235/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1235(t2104: f64, t2107: f64, t2946: f64, t54: f64, t2027: f64, t2739: f64, t1066: f64, t5726: f64, t17897: f64, t17902: f64, t17903: f64, t17905: f64, t2031: f64, t2039: f64, t2096: f64, t2105: f64, t2899: f64, t2922: f64, t2976: f64, t5693: f64, t5729: f64, t5934: f64, t5956: f64, t5984: f64, t7578: f64, t7640: f64, t7664: f64, t7696: f64, t7736: f64, t7742: f64) -> f64 {
    let t21567 = t2104 * t54 * t2946 * t2107;
    let t21569 = t2739 * t2027;
    let t21578 = t1066 * t5726;
    let t21597 = -0.10162730220579493208e-2_f64 * t17897 - t17902 - 0.35400176935018568009e-1_f64 * t17903 - 0.48272968547752592738e-2_f64 * t17905 - 0.41159057393346947493e-1_f64 * t5984 * t7696 + 0.51448821741683684367e-2_f64 * t21567 - 0.25724410870841842183e-2_f64 * t2899 * t2105 * t21569 * t2031 + 0.12862205435420921092e-2_f64 * t2922 * t2105 * t21569 * t2039 - 0.25724410870841842183e-2_f64 * t7736 * t2105 * t21578 * t5956 + 0.25724410870841842183e-2_f64 * t7742 * t2105 * t21578 * t5729 - 0.42874018118069736972e-3_f64 * t7664 * t2105 * t21578 * t5934 - 0.34299214494455789577e-2_f64 * t2096 * t7578 + 0.38586616306262763275e-2_f64 * t2104 * t5693 * t2976 * t7640;
    t21597
}
