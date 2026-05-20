//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3739/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3739<F: Float>(t1250: F, t5245: F, t1794: F, t372: F, t5277: F, t17395: F, t17400: F, t12702: F, t12744: F, t12787: F, t12855: F, t12910: F, t16725: F, t17354: F, t17657: F, t17688: F, t17693: F, t20795: F, t20945: F, t20946: F, t20947: F, t21013: F, t21119: F, t21164: F, t3617: F, t3720: F, t3723: F, t44510: F, t44517: F, t5284: F, t5331: F, t5335: F, t5343: F, t5346: F, t56888: F, t58824: F, t58827: F) -> (F, F) {
    let t71055 = t1250 * t5245;
    let t71061 = t372 * t5277 * t1794;
    let t71081 = t17400 * t17395;
    let t71098 = F::cast_from(0.17149607247227894789e-2_f64) * t12910 * t3720 * t5346 * t71055 + F::cast_from(0.11433071498151929859e-2_f64) * t44510 * t71061 * t17657 - F::cast_from(0.57165357490759649296e-3_f64) * t44517 * t71061 * t17354 - F::cast_from(0.23818898954483187207e-3_f64) * t5331 * t12787 * t20795 * t17688 - F::cast_from(0.17149607247227894789e-2_f64) * t12855 * t3720 * t21164 * t21119 + F::cast_from(0.1270341277572436651e-3_f64) * t58824 - F::cast_from(0.7622047665434619906e-3_f64) * t58827 - F::cast_from(0.91464571985215438872e-2_f64) * t12702 * t21013 * t5343 + F::cast_from(0.45732285992607719436e-2_f64) * t71081 * t3723 + F::cast_from(0.45732285992607719436e-2_f64) * t12744 * t21013 * t5335 + F::cast_from(0.95275595817932748826e-3_f64) * t56888 * t20947 + F::cast_from(0.95275595817932748826e-3_f64) * t17693 * t372 * t3617 * t5284 * t20946 + F::cast_from(0.95275595817932748826e-3_f64) * t17693 * t20945 * t1250 * t16725;
    (t71061, t71098)
}
