//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1074/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1074<F: Float>(t30226: F, t30230: F, t30233: F, t30239: F, t30240: F, t30243: F, t30247: F, t30249: F, t33963: F, t33983: F, t33987: F, t33995: F, t36876: F, t36889: F, t38890: F, t38894: F, t38899: F, t38903: F) -> F {
    let t38905 = F::new(0.22921875e-1) * t38890 + F::new(0.1528125e-1) * t38894 - t36876 + t33963 + F::cast_from(0.85748036236139473944e-3_f64) * t30226 + t30230 + t30233 + t30239 + F::cast_from(0.10718504529517434243e-3_f64) * t30240 + t30243 - t30247 - F::cast_from(0.45351183609335988444e-1_f64) * t30249 + F::cast_from(0.42874018118069736972e-3_f64) * t38899 - t33983 + t36889 + t33987 + F::cast_from(0.18868855373762491241e-2_f64) * t38903 + t33995;
    t38905
}
