//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1008/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1008<F: Float>(t3201: F, t8489: F, t1980: F, t7458: F, t30046: F, t30048: F, t30051: F, t30056: F, t30061: F, t30073: F, t30078: F, t30081: F, t30084: F, t33872: F, t33874: F, t33876: F, t33881: F, t33887: F, t33890: F, t33894: F, t33898: F) -> (F, F) {
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33904 = F::cast_from(0.28582678745379824648e-3_f64) * t33903;
    let t33905 = -t33872 + t30046 + t30048 + t30051 + t30056 + F::cast_from(0.21437009059034868486e-2_f64) * t30061 - F::cast_from(0.53592522647587171215e-3_f64) * t33874 - F::cast_from(0.90035438047946447644e-2_f64) * t33876 - F::cast_from(0.21437009059034868486e-3_f64) * t33881 - t33887 - F::cast_from(0.42874018118069736972e-3_f64) * t33890 - F::cast_from(0.14291339372689912324e-3_f64) * t33894 - F::cast_from(0.42874018118069736972e-3_f64) * t33898 - F::cast_from(0.85748036236139473944e-3_f64) * t30073 - t33904 - t30078 - t30081 + t30084;
    (t33901, t33905)
}
