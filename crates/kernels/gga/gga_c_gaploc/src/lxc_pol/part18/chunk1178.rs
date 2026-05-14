//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1178/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1178<F: Float>(t1890: F, t32356: F, t1966: F, t590: F, t28878: F, t28880: F, t10668: F, t10996: F, t11083: F, t11102: F, t1445: F, t1589: F, t2159: F, t2197: F, t28862: F, t28864: F, t28866: F, t28873: F, t28876: F, t28891: F, t317: F, t32180: F, t3465: F, t3508: F, t5724: F, t6024: F, t769: F, t797: F, t807: F) -> (F,) {
    let t33760 = t1890 * t32356;
    let t33763 = 0.51123901271894332902e1 * t1966 * t33760 * t590;
    let t33773 = 0.12780975317973583226e0 * t28878;
    let t33774 = 0.63904876589867916128e-1 * t28880;
    let t33777 = -0.47667319935800568892e0 * t797 * t1589 * t10668 - t28862 + t28864 - t28866 + t28873 + t28876 + 0.46011511144704899612e1 * t807 * t1445 * t32180 - t33763 + 0.71500979903700853338e0 * t769 * t11083 * t317 + 0.23005755572352449806e1 * t6024 * t3508 - 0.79445533226334281487e-1 * t3465 * t2159 + 0.46011511144704899612e1 * t2197 * t11102 - t33773 + t33774 + t28891 - 0.35750489951850426669e0 * t10996 * t5724;
    (t33777,)
}
