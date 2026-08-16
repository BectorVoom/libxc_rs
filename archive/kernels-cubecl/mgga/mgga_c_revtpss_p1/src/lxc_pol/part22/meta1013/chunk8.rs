//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3487/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3487<F: Float>(t1042: F, t1047: F, t11656: F, t15707: F, t15830: F, t16167: F, t16172: F, t19792: F, t19934: F, t19940: F, t3106: F, t3161: F, t3162: F, t42371: F, t4808: F, t6312: F, t65482: F, t65596: F, t65598: F, t65610: F, t65613: F, t65618: F) -> F {
    let t65626 = F::cast_from(0.6351706387862183255e-4_f64) * t65596 + F::cast_from(0.57165357490759649296e-3_f64) * t65598 + F::cast_from(0.30488190661738479624e-2_f64) * t11656 * t19940 - F::cast_from(0.42874018118069736972e-3_f64) * t3161 * t1042 * t65482 * t3162 + F::cast_from(0.60976381323476959249e-2_f64) * t3106 * t19934 - F::cast_from(0.5081365110289746604e-2_f64) * t15830 * t4808 + F::cast_from(0.6351706387862183255e-3_f64) * t65610 + F::cast_from(0.42874018118069736972e-3_f64) * t65613 * t1047 + F::cast_from(0.22866142996303859718e-2_f64) * t42371 * t6312 - F::cast_from(0.28582678745379824648e-3_f64) * t65618 - F::cast_from(0.28582678745379824648e-3_f64) * t15707 * t16167 - F::cast_from(0.47637797908966374413e-3_f64) * t15707 * t16172 + F::cast_from(0.30488190661738479624e-2_f64) * t11656 * t19792;
    t65626
}
