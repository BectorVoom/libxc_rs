//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 982/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk982(t758: f64, t7788: f64, t1843: f64, t2946: f64, t1066: f64, t5633: f64, t1885: f64, t1130: f64, t2082: f64, t2096: f64, t2899: f64, t2919: f64, t2922: f64, t2945: f64, t2952: f64, t299: f64, t5709: f64, t5922: f64, t5929: f64, t5941: f64, t5948: f64, t5976: f64, t771: f64, t7751: f64, t7756: f64, t7760: f64, t7767: f64, t7770: f64, t7776: f64, t7786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7789 = t758 * t7788;
    let t7792 = t2946 * t1843;
    let t7793 = t758 * t7792;
    let t7796 = t5633 * t1066;
    let t7797 = t7796 * t1885;
    let t7798 = t758 * t7797;
    let t7802 = -0.42874018118069736972e-3_f64 * t299 * t7751 + 0.95275595817932748827e-4_f64 * t7756 - 0.14481890564325777821e-1_f64 * t2082 * t1130 + 0.30488190661738479624e-2_f64 * t7760 + 0.45732285992607719436e-2_f64 * t771 * t2952 - t7767 - 0.17149607247227894789e-2_f64 * t2899 * t7770 - 0.22866142996303859718e-2_f64 * t2096 * t2919 + 0.42874018118069736972e-3_f64 * t2922 * t7776 - 0.14291339372689912324e-3_f64 * t5709 + 0.14291339372689912324e-3_f64 * t5922 + 0.28582678745379824648e-3_f64 * t5929 - 0.95275595817932748826e-4_f64 * t5941 - 0.15244095330869239812e-2_f64 * t5948 + t7786 + 0.25724410870841842184e-2_f64 * t2945 * t7789 + 0.12862205435420921092e-2_f64 * t2945 * t7793 - 0.51448821741683684368e-2_f64 * t2945 * t7798 - 0.57165357490759649296e-3_f64 * t5976;
    (t7789, t7792, t7793, t7796, t7797, t7798, t7802)
}
