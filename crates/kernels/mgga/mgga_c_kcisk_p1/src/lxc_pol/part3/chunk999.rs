//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 999/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk999<F: Float>(t14190: F, t14193: F, t14195: F, t14201: F, t14206: F, t14211: F, t14216: F, t14218: F, t14220: F, t14224: F, t14226: F, t14228: F) -> F {
    let t14715 = F::new(0.10446625e-1) * t14190 + F::cast_from(0.27857666666666666666e-1_f64) * t14193 + F::cast_from(0.46429444444444444443e-2_f64) * t14195 + F::cast_from(0.18571777777777777778e-1_f64) * t14201 - F::cast_from(0.34822083333333333333e-2_f64) * t14206 + F::cast_from(0.51588271604938271604e-3_f64) * t14211 + F::cast_from(0.30952962962962962963e-2_f64) * t14216 + F::cast_from(0.23214722222222222222e-2_f64) * t14218 - F::cast_from(0.69644166666666666665e-2_f64) * t14220 + F::cast_from(0.11607361111111111111e-2_f64) * t14224 - F::cast_from(0.77382407407407407405e-3_f64) * t14226 - F::cast_from(0.12381185185185185185e-1_f64) * t14228;
    t14715
}
