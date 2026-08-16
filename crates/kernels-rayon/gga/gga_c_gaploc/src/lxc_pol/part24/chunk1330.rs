//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1330/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1330(t1890: f64, t32356: f64, t1966: f64, t590: f64, t28878: f64, t28880: f64, t10668: f64, t10996: f64, t11083: f64, t11102: f64, t1445: f64, t1589: f64, t2159: f64, t2197: f64, t28862: f64, t28864: f64, t28866: f64, t28873: f64, t28876: f64, t28891: f64, t317: f64, t32180: f64, t3465: f64, t3508: f64, t5724: f64, t6024: f64, t769: f64, t797: f64, t807: f64) -> f64 {
    let t33760 = t1890 * t32356;
    let t33763 = 0.51123901271894332902e1_f64 * t1966 * t33760 * t590;
    let t33773 = 0.12780975317973583226e0_f64 * t28878;
    let t33774 = 0.63904876589867916128e-1_f64 * t28880;
    let t33777 = -0.47667319935800568892e0_f64 * t797 * t1589 * t10668 - t28862 + t28864 - t28866 + t28873 + t28876 + 0.46011511144704899612e1_f64 * t807 * t1445 * t32180 - t33763 + 0.71500979903700853338e0_f64 * t769 * t11083 * t317 + 0.23005755572352449806e1_f64 * t6024 * t3508 - 0.79445533226334281487e-1_f64 * t3465 * t2159 + 0.46011511144704899612e1_f64 * t2197 * t11102 - t33773 + t33774 + t28891 - 0.35750489951850426669e0_f64 * t10996 * t5724;
    t33777
}
