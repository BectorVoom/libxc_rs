//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3541/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3541(t11774: f64, t4787: f64, t53391: f64, t1062: f64, t19857: f64, t1011: f64, t1042: f64, t1068: f64, t15716: f64, t16012: f64, t16152: f64, t1663: f64, t19864: f64, t42425: f64, t43066: f64, t43204: f64, t43215: f64, t55058: f64, t55061: f64, t55064: f64, t55067: f64, t55070: f64, t55072: f64, t6263: f64, t63302: f64) -> f64 {
    let t67264 = t11774 * t53391 * t4787;
    let t67269 = t19857 * t1062;
    let t67283 = -0.19055119163586549765e-3_f64 * t55058 + 0.2540682555144873302e-3_f64 * t55061 - 0.21172354626207277516e-3_f64 * t55064 + 0.30488190661738479624e-2_f64 * t43066 * t19864 - 0.3811023832717309953e-3_f64 * t67264 - 0.57165357490759649296e-3_f64 * t55067 - 0.20325460441158986416e-2_f64 * t55070 + 0.64363958063670123651e-2_f64 * t55072 + 0.28582678745379824648e-3_f64 * t67269 * t1068 + 0.6351706387862183255e-4_f64 * t43204 - 0.96545937095505185476e-2_f64 * t42425 * t6263 - 0.51448821741683684367e-2_f64 * t15716 * t1042 * t1663 * t16152 - 7.0_f64 / 54.0_f64 * t1011 * t16012 * t63302 + 0.33875767401931644026e-3_f64 * t43215;
    t67283
}
