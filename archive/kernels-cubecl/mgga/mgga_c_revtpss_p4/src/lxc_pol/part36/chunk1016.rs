//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1016/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1016<F: Float>(t23535: F, t916: F, t923: F, t1600: F, t6113: F, t11354: F, t11358: F, t11334: F, t11338: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F) -> (F, F, F, F, F) {
    let t23536 = t916 * t23535;
    let t23538 = t923 * t23535;
    let t23540 = t6113 * t1600;
    let t23541 = t11354 * t23540;
    let t23543 = t11358 * t23540;
    let t23545 = F::cast_from(0.19931111111111111111e0_f64) * t18919 - F::cast_from(0.59793333333333333333e0_f64) * t18924 + F::cast_from(0.29896666666666666667e0_f64) * t18934 - t11334 - t11338 + F::cast_from(0.5477111111111111111e-1_f64) * t19002 - F::cast_from(0.32862666666666666666e0_f64) * t19004 + F::cast_from(0.16431333333333333333e0_f64) * t19009 - F::cast_from(0.28483875e1_f64) * t23521 + F::cast_from(0.46074375e0_f64) * t23523 + F::cast_from(0.1898925e1_f64) * t23536 + F::cast_from(0.3071625e0_f64) * t23538 + F::cast_from(0.142419375e1_f64) * t23541 - F::cast_from(0.76790625e-1_f64) * t23543;
    (t23536, t23538, t23541, t23543, t23545)
}
