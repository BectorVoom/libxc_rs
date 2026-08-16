//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 768/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk768<F: Float>(t7805: F, t7849: F, t7853: F, t7862: F, t7809: F, t7813: F, t7817: F, t7820: F, t7823: F, t7825: F, t7829: F, t7833: F, t7837: F, t7840: F, t7845: F, t7847: F, t7856: F, t7864: F, t7868: F, t7872: F) -> (F, F, F, F, F) {
    let t8278 = F::cast_from(0.41930789719472202758e-3_f64) * t7805;
    let t8291 = F::cast_from(77.0_f64) / F::cast_from(864.0_f64) * t7849;
    let t8292 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t7853;
    let t8294 = t7862 / F::cast_from(192.0_f64);
    let t8298 = -t8278 + F::cast_from(0.22921875e-1_f64) * t7809 + F::cast_from(0.1528125e-1_f64) * t7813 + t7817 / F::cast_from(32.0_f64) + F::cast_from(0.4584375e-1_f64) * t7820 - F::cast_from(0.34299214494455789578e-2_f64) * t7823 + F::cast_from(0.34299214494455789578e-2_f64) * t7825 - t7829 / F::cast_from(64.0_f64) + F::cast_from(0.31448092289604152069e-3_f64) * t7833 + F::cast_from(0.42874018118069736972e-3_f64) * t7837 + F::cast_from(0.62896184579208304138e-3_f64) * t7840 + F::cast_from(0.41930789719472202758e-3_f64) * t7845 - F::cast_from(0.42874018118069736972e-3_f64) * t7847 + t8291 + t8292 + t7856 / F::cast_from(48.0_f64) + t8294 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t7864 + F::cast_from(0.62896184579208304137e-2_f64) * t7868 - F::cast_from(0.94344276868812456206e-2_f64) * t7872;
    (t8278, t8291, t8292, t8294, t8298)
}
