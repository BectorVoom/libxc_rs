//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1045/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1045<F: Float>(t33962: F, t33982: F, t33986: F, t33994: F, t33996: F, t34013: F, t34027: F, t34031: F, t34035: F, t34037: F, t34052: F, t34081: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36877 = F::new(11.0) / F::new(96.0) * t33962;
    let t36888 = F::new(0.12862205435420921092e-1) * t33982;
    let t36890 = F::new(0.12579236915841660828e-2) * t33986;
    let t36892 = F::new(0.14291339372689912324e-2) * t33994;
    let t36893 = F::new(0.85748036236139473944e-3) * t33996;
    let t36900 = F::new(0.12862205435420921092e-1) * t34013;
    let t36908 = F::new(0.42874018118069736972e-2) * t34027;
    let t36910 = F::new(0.2264262644851498949e-1) * t34031;
    let t36912 = F::new(0.42874018118069736972e-3) * t34035;
    let t36913 = F::new(0.42874018118069736972e-3) * t34037;
    let t36918 = F::new(0.14291339372689912324e-2) * t34052;
    let t36934 = F::new(0.31448092289604152068e-2) * t34081;
    (t36877, t36888, t36890, t36892, t36893, t36900, t36908, t36910, t36912, t36913, t36918, t36934)
}
