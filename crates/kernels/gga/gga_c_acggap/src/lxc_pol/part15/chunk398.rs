//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 398/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk398<F: Float>(t1572: F, t1584: F, t1591: F, t1593: F, t1595: F, t1597: F, t1599: F, t1881: F, t1886: F, t1891: F, t1896: F, t1901: F, t418: F) -> F {
    let t1905 = F::cast_from(0.85748036236139473944e-3_f64) * t1572 - F::cast_from(0.85748036236139473944e-3_f64) * t1584 - F::cast_from(0.40015750243531754508e-2_f64) * t1591 + F::cast_from(0.40015750243531754508e-2_f64) * t1593 + F::cast_from(0.80031500487063509015e-2_f64) * t1595 - F::new(7.0) / F::new(144.0) * t1597 + F::cast_from(0.12862205435420921092e-2_f64) * t418 * t1881 + F::cast_from(0.42874018118069736972e-2_f64) * t418 * t1886 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t1891 + F::cast_from(0.42874018118069736972e-3_f64) * t418 * t1896 - F::cast_from(0.42874018118069736972e-3_f64) * t418 * t1901 + F::cast_from(0.20007875121765877254e-2_f64) * t1599;
    t1905
}
