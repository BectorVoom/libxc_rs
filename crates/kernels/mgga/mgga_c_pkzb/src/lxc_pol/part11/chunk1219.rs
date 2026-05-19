//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1219/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1219<F: Float>(t10767: F, t779: F, t2003: F, t300: F, t3638: F, t1066: F, t2916: F, t5955: F, t10932: F, t10995: F, t11039: F, t11070: F, t17957: F, t2031: F, t2039: F, t2104: F, t2105: F, t2106: F, t21455: F, t21729: F, t25337: F, t25518: F, t25530: F, t25553: F, t2739: F, t2774: F, t2887: F, t2888: F, t2889: F, t2899: F, t2901: F, t2922: F, t2923: F, t2931: F, t29775: F, t302: F, t3515: F, t3685: F, t5633: F, t5956: F, t5984: F, t655: F, t7592: F, t7664: F, t7700: F, t7707: F, t7736: F, t9161: F, t9277: F, t9282: F, t9296: F, t9575: F) -> (F, F, F, F) {
    let t29996 = t779 * t10767;
    let t30002 = t300 * t2003 * t3638;
    let t30013 = t1066 * t2916;
    let t30038 = t5955 * t2916;
    let t30044 = -F::cast_from(0.68598428988911579157e-2_f64) * t7707 * t10995 + F::cast_from(0.68598428988911579157e-2_f64) * t5984 * t11039 - F::cast_from(0.34299214494455789578e-2_f64) * t25518 + t2887 * t2888 * t5633 * t10932 * t655 / F::new(4.0) - F::new(3.0) / F::new(16.0) * t2887 * t2888 * t9296 * t2739 + t2887 * t2888 * t7592 * t3515 / F::new(16.0) + t2887 * t2888 * t2889 * t9161 / F::new(16.0) + t2887 * t2888 * t29996 * t655 / F::new(48.0) - F::cast_from(0.38586616306262763276e-2_f64) * t2922 * t30002 * t2039 * t2774 - F::cast_from(0.77173232612525526549e-2_f64) * t7736 * t25337 * t5956 * t2931 - F::cast_from(0.22866142996303859718e-2_f64) * t25530 + F::cast_from(0.63517063878621832551e-4_f64) * t17957 - F::cast_from(0.51448821741683684366e-2_f64) * t2899 * t7700 * t2031 * t30013 - F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t2105 * t3685 * t9575 - F::cast_from(0.42874018118069736972e-3_f64) * t7664 * t2105 * t11070 * t2106 + F::cast_from(0.64311027177104605458e-3_f64) * t7664 * t302 * t9282 * t9277 + F::cast_from(0.30011812682648815881e-2_f64) * t21455 * t302 * t29775 * t2901 - F::cast_from(0.21437009059034868486e-3_f64) * t21729 * t302 * t29775 * t2923 + F::cast_from(0.38586616306262763276e-2_f64) * t7736 * t302 * t9282 * t30038 + F::cast_from(0.22866142996303859718e-2_f64) * t25553;
    (t30002, t30013, t30038, t30044)
}
