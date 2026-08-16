//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1271/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1271<F: Float>(t1881: F, t3228: F, t1901: F, t3237: F, t1008: F, t6102: F, t1137: F, t5590: F, t1089: F, t12610: F, t14022: F, t18072: F, t18079: F, t18085: F, t18087: F, t1849: F, t1899: F, t21771: F, t3266: F, t386: F, t387: F, t418: F, t435: F) -> F {
    let t23480 = t3228 * t1881;
    let t23482 = t3237 * t1901;
    let t23484 = t3228 * t1901;
    let t23486 = t1008 * t6102;
    let t23494 = t1137 * t5590;
    let t23499 = F::cast_from(0.20007875121765877254e-2_f64) * t14022 + F::cast_from(0.68598428988911579156e-2_f64) * t418 * t1089 * t12610 * t1849 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t386 * t3266 * t1899 + F::cast_from(0.12862205435420921092e-2_f64) * t23480 + F::cast_from(0.40015750243531754508e-2_f64) * t23482 - F::cast_from(0.42874018118069736972e-3_f64) * t23484 - F::cast_from(0.85748036236139473944e-3_f64) * t23486 - F::cast_from(0.42874018118069736972e-3_f64) * t418 * t386 * t387 * t435 * t21771 - F::cast_from(0.16006300097412701803e-1_f64) * t18072 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t23494 + F::cast_from(7.0_f64) / F::cast_from(6.0_f64) * t18079 + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t18085 + F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t18087;
    t23499
}
