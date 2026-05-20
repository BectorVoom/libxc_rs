//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3699/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3699<F: Float>(t21007: F, t3625: F, t44425: F, t12772: F, t21222: F, t5340: F, t21101: F, t3707: F, t1252: F, t12855: F, t17222: F, t17426: F, t17454: F, t17456: F, t17633: F, t1797: F, t20797: F, t20800: F, t20956: F, t20963: F, t21020: F, t21223: F, t3626: F, t3720: F, t44252: F, t44578: F, t44664: F, t5293: F, t57707: F, t59375: F, t59401: F) -> F {
    let t70064 = t3625 * t44425 * t21007;
    let t70076 = t5340 * t12772 * t21222;
    let t70082 = t3707 * t21101;
    let t70085 = -F::cast_from(0.57165357490759649296e-3_f64) * t17426 * t21223 - F::cast_from(0.57165357490759649296e-3_f64) * t3625 * t3626 * t17633 * t21020 + F::cast_from(0.91464571985215438872e-2_f64) * t57707 * t17456 - F::cast_from(0.25724410870841842184e-2_f64) * t59401 * t20963 + F::cast_from(0.42874018118069736972e-3_f64) * t44664 * t20797 + F::cast_from(0.31758531939310916276e-3_f64) * t70064 + F::cast_from(0.25724410870841842183e-2_f64) * t44578 * t3720 * t20956 * t17454 - F::cast_from(0.85748036236139473944e-3_f64) * t12855 * t3720 * t20800 * t17454 + F::cast_from(0.6351706387862183255e-4_f64) * t44252 - F::cast_from(0.3811023832717309953e-3_f64) * t70076 - F::cast_from(0.22866142996303859718e-2_f64) * t5293 * t17222 + F::cast_from(0.42874018118069736972e-3_f64) * t59375 * t1797 + F::cast_from(0.14481890564325777821e-1_f64) * t70082 * t1252;
    t70085
}
