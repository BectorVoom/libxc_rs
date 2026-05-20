//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1136/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1136<F: Float>(t119836: F, t126322: F, t119875: F, t33678: F, t119883: F, t119941: F, t120002: F, t120007: F, t120014: F, t120017: F, t120044: F, t120048: F, t120063: F, t126291: F, t126304: F, t126319: F, t126323: F, t1949: F, t27265: F, t27322: F, t31794: F, t31814: F, t31820: F, t32426: F, t33698: F, t33699: F, t33708: F, t34075: F, t8649: F, t8650: F, t886: F) -> F {
    let t126325 = t119836 * t126322;
    let t126327 = t119875 * t33678;
    let t126333 = F::cast_from(0.8673628188205199462e0_f64) * t31794 * t126291 - F::cast_from(0.33059535666846348619e-4_f64) * t120002 + F::cast_from(0.34694512752820797848e1_f64) * t119941 * t27322 - F::cast_from(0.50779446784275991476e-1_f64) * t120007 - F::cast_from(0.11423947533020470523e1_f64) * t34075 * t31820 + F::cast_from(0.6854368519812282314e1_f64) * t8649 * t119883 * t33698 * t886 - F::cast_from(0.17347256376410398924e1_f64) * t31794 * t126304 + F::cast_from(0.11423947533020470523e1_f64) * t8649 * t8650 * t1949 * t27265 + F::cast_from(0.18822977838986977999e-4_f64) * t120014 - F::cast_from(0.33467254597718846885e-4_f64) * t120017 - F::cast_from(0.17135921299530705785e1_f64) * t32426 * t33699 + F::cast_from(0.11423947533020470523e1_f64) * t32426 * t33708 - F::cast_from(0.112937867033921868e-2_f64) * t126319 + F::cast_from(0.75291911355947911999e-4_f64) * t126323 - F::cast_from(0.13386901839087538754e-3_f64) * t126325 + F::cast_from(0.527043379491635384e-2_f64) * t126327 - F::cast_from(0.7437465841810202164e-4_f64) * t120044 - F::cast_from(0.34708173928447610099e-2_f64) * t120048 - t120063 - F::cast_from(0.17135921299530705785e1_f64) * t34075 * t31814;
    t126333
}
