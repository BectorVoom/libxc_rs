//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 982/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk982<F: Float>(t758: F, t7788: F, t1843: F, t2946: F, t1066: F, t5633: F, t1885: F, t1130: F, t2082: F, t2096: F, t2899: F, t2919: F, t2922: F, t2945: F, t2952: F, t299: F, t5709: F, t5922: F, t5929: F, t5941: F, t5948: F, t5976: F, t771: F, t7751: F, t7756: F, t7760: F, t7767: F, t7770: F, t7776: F, t7786: F) -> (F, F, F, F, F, F, F) {
    let t7789 = t758 * t7788;
    let t7792 = t2946 * t1843;
    let t7793 = t758 * t7792;
    let t7796 = t5633 * t1066;
    let t7797 = t7796 * t1885;
    let t7798 = t758 * t7797;
    let t7802 = -F::new(0.42874018118069736972e-3) * t299 * t7751 + F::new(0.95275595817932748827e-4) * t7756 - F::new(0.14481890564325777821e-1) * t2082 * t1130 + F::new(0.30488190661738479624e-2) * t7760 + F::new(0.45732285992607719436e-2) * t771 * t2952 - t7767 - F::new(0.17149607247227894789e-2) * t2899 * t7770 - F::new(0.22866142996303859718e-2) * t2096 * t2919 + F::new(0.42874018118069736972e-3) * t2922 * t7776 - F::new(0.14291339372689912324e-3) * t5709 + F::new(0.14291339372689912324e-3) * t5922 + F::new(0.28582678745379824648e-3) * t5929 - F::new(0.95275595817932748826e-4) * t5941 - F::new(0.15244095330869239812e-2) * t5948 + t7786 + F::new(0.25724410870841842184e-2) * t2945 * t7789 + F::new(0.12862205435420921092e-2) * t2945 * t7793 - F::new(0.51448821741683684368e-2) * t2945 * t7798 - F::new(0.57165357490759649296e-3) * t5976;
    (t7789, t7792, t7793, t7796, t7797, t7798, t7802)
}
