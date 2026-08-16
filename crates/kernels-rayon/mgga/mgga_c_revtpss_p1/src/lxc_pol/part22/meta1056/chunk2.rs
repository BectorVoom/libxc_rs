//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3739/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3739(t1250: f64, t5245: f64, t1794: f64, t372: f64, t5277: f64, t17395: f64, t17400: f64, t12702: f64, t12744: f64, t12787: f64, t12855: f64, t12910: f64, t16725: f64, t17354: f64, t17657: f64, t17688: f64, t17693: f64, t20795: f64, t20945: f64, t20946: f64, t20947: f64, t21013: f64, t21119: f64, t21164: f64, t3617: f64, t3720: f64, t3723: f64, t44510: f64, t44517: f64, t5284: f64, t5331: f64, t5335: f64, t5343: f64, t5346: f64, t56888: f64, t58824: f64, t58827: f64) -> (f64, f64) {
    let t71055 = t1250 * t5245;
    let t71061 = t372 * t5277 * t1794;
    let t71081 = t17400 * t17395;
    let t71098 = 0.17149607247227894789e-2_f64 * t12910 * t3720 * t5346 * t71055 + 0.11433071498151929859e-2_f64 * t44510 * t71061 * t17657 - 0.57165357490759649296e-3_f64 * t44517 * t71061 * t17354 - 0.23818898954483187207e-3_f64 * t5331 * t12787 * t20795 * t17688 - 0.17149607247227894789e-2_f64 * t12855 * t3720 * t21164 * t21119 + 0.1270341277572436651e-3_f64 * t58824 - 0.7622047665434619906e-3_f64 * t58827 - 0.91464571985215438872e-2_f64 * t12702 * t21013 * t5343 + 0.45732285992607719436e-2_f64 * t71081 * t3723 + 0.45732285992607719436e-2_f64 * t12744 * t21013 * t5335 + 0.95275595817932748826e-3_f64 * t56888 * t20947 + 0.95275595817932748826e-3_f64 * t17693 * t372 * t3617 * t5284 * t20946 + 0.95275595817932748826e-3_f64 * t17693 * t20945 * t1250 * t16725;
    (t71061, t71098)
}
