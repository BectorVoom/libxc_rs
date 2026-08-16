//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1045/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1045(t33962: f64, t33982: f64, t33986: f64, t33994: f64, t33996: f64, t34013: f64, t34027: f64, t34031: f64, t34035: f64, t34037: f64, t34052: f64, t34081: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36877 = 11.0_f64 / 96.0_f64 * t33962;
    let t36888 = 0.12862205435420921092e-1_f64 * t33982;
    let t36890 = 0.12579236915841660828e-2_f64 * t33986;
    let t36892 = 0.14291339372689912324e-2_f64 * t33994;
    let t36893 = 0.85748036236139473944e-3_f64 * t33996;
    let t36900 = 0.12862205435420921092e-1_f64 * t34013;
    let t36908 = 0.42874018118069736972e-2_f64 * t34027;
    let t36910 = 0.2264262644851498949e-1_f64 * t34031;
    let t36912 = 0.42874018118069736972e-3_f64 * t34035;
    let t36913 = 0.42874018118069736972e-3_f64 * t34037;
    let t36918 = 0.14291339372689912324e-2_f64 * t34052;
    let t36934 = 0.31448092289604152068e-2_f64 * t34081;
    (t36877, t36888, t36890, t36892, t36893, t36900, t36908, t36910, t36912, t36913, t36918, t36934)
}
