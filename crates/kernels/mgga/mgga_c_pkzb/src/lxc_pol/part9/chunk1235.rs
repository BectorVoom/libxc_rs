//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1235/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1235<F: Float>(t2104: F, t2107: F, t2946: F, t54: F, t2027: F, t2739: F, t1066: F, t5726: F, t17897: F, t17902: F, t17903: F, t17905: F, t2031: F, t2039: F, t2096: F, t2105: F, t2899: F, t2922: F, t2976: F, t5693: F, t5729: F, t5934: F, t5956: F, t5984: F, t7578: F, t7640: F, t7664: F, t7696: F, t7736: F, t7742: F) -> F {
    let t21567 = t2104 * t54 * t2946 * t2107;
    let t21569 = t2739 * t2027;
    let t21578 = t1066 * t5726;
    let t21597 = -F::new(0.10162730220579493208e-2) * t17897 - t17902 - F::new(0.35400176935018568009e-1) * t17903 - F::new(0.48272968547752592738e-2) * t17905 - F::new(0.41159057393346947493e-1) * t5984 * t7696 + F::new(0.51448821741683684367e-2) * t21567 - F::new(0.25724410870841842183e-2) * t2899 * t2105 * t21569 * t2031 + F::new(0.12862205435420921092e-2) * t2922 * t2105 * t21569 * t2039 - F::new(0.25724410870841842183e-2) * t7736 * t2105 * t21578 * t5956 + F::new(0.25724410870841842183e-2) * t7742 * t2105 * t21578 * t5729 - F::new(0.42874018118069736972e-3) * t7664 * t2105 * t21578 * t5934 - F::new(0.34299214494455789577e-2) * t2096 * t7578 + F::new(0.38586616306262763275e-2) * t2104 * t5693 * t2976 * t7640;
    t21597
}
