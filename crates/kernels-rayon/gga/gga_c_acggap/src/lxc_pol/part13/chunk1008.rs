//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1008/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1008(t3201: f64, t8489: f64, t1980: f64, t7458: f64, t30046: f64, t30048: f64, t30051: f64, t30056: f64, t30061: f64, t30073: f64, t30078: f64, t30081: f64, t30084: f64, t33872: f64, t33874: f64, t33876: f64, t33881: f64, t33887: f64, t33890: f64, t33894: f64, t33898: f64) -> (f64, f64) {
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33904 = 0.28582678745379824648e-3_f64 * t33903;
    let t33905 = -t33872 + t30046 + t30048 + t30051 + t30056 + 0.21437009059034868486e-2_f64 * t30061 - 0.53592522647587171215e-3_f64 * t33874 - 0.90035438047946447644e-2_f64 * t33876 - 0.21437009059034868486e-3_f64 * t33881 - t33887 - 0.42874018118069736972e-3_f64 * t33890 - 0.14291339372689912324e-3_f64 * t33894 - 0.42874018118069736972e-3_f64 * t33898 - 0.85748036236139473944e-3_f64 * t30073 - t33904 - t30078 - t30081 + t30084;
    (t33901, t33905)
}
