//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1508/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1508<F: Float>(t10673: F, t10676: F, t14668: F, t14675: F, t14678: F, t14682: F, t14690: F, t14693: F, t14697: F, t14703: F, t14705: F, t14707: F, t2745: F, t4362: F) -> F {
    let t14711 = F::cast_from(0.42874018118069736972e-3_f64) * t4362 * t14668 + t14675 - F::cast_from(0.42874018118069736972e-3_f64) * t2745 * t14678 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t14682 - t14690 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t14693 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t14697 + t14703 + t14705 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t14707 + t10673 - F::cast_from(0.14291339372689912324e-3_f64) * t10676;
    t14711
}
