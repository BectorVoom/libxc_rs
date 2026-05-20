//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3541/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3541<F: Float>(t11774: F, t4787: F, t53391: F, t1062: F, t19857: F, t1011: F, t1042: F, t1068: F, t15716: F, t16012: F, t16152: F, t1663: F, t19864: F, t42425: F, t43066: F, t43204: F, t43215: F, t55058: F, t55061: F, t55064: F, t55067: F, t55070: F, t55072: F, t6263: F, t63302: F) -> F {
    let t67264 = t11774 * t53391 * t4787;
    let t67269 = t19857 * t1062;
    let t67283 = -F::cast_from(0.19055119163586549765e-3_f64) * t55058 + F::cast_from(0.2540682555144873302e-3_f64) * t55061 - F::cast_from(0.21172354626207277516e-3_f64) * t55064 + F::cast_from(0.30488190661738479624e-2_f64) * t43066 * t19864 - F::cast_from(0.3811023832717309953e-3_f64) * t67264 - F::cast_from(0.57165357490759649296e-3_f64) * t55067 - F::cast_from(0.20325460441158986416e-2_f64) * t55070 + F::cast_from(0.64363958063670123651e-2_f64) * t55072 + F::cast_from(0.28582678745379824648e-3_f64) * t67269 * t1068 + F::cast_from(0.6351706387862183255e-4_f64) * t43204 - F::cast_from(0.96545937095505185476e-2_f64) * t42425 * t6263 - F::cast_from(0.51448821741683684367e-2_f64) * t15716 * t1042 * t1663 * t16152 - F::new(7.0) / F::new(54.0) * t1011 * t16012 * t63302 + F::cast_from(0.33875767401931644026e-3_f64) * t43215;
    t67283
}
