//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 986/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk986<F: Float>(t11924: F, t11927: F, t11930: F, t11933: F, t11938: F, t11941: F, t11944: F, t11947: F, t11952: F, t11954: F, t11956: F, t11962: F, t11965: F, t11967: F, t11972: F, t11973: F, t3120: F, t3208: F, t375: F) -> F {
    let t11976 = -F::new(0.85748036236139473944e-3) * t11924 + F::new(0.12862205435420921092e-2) * t11927 * t11930 + F::new(0.68598428988911579154e-2) * t11933 * t3120 + F::new(0.85748036236139473944e-3) * t11938 - F::new(0.12862205435420921092e-2) * t11941 * t11944 - F::new(0.68598428988911579154e-2) * t11947 * t3208 - F::new(0.42874018118069736972e-3) * t11952 - F::new(0.45732285992607719436e-2) * t11954 - F::new(0.14291339372689912324e-3) * t11956 - F::new(0.53100265402527852012e-1) * t11962 * t375 + F::new(0.14481890564325777821e-1) * t11965 + F::new(0.7622047665434619906e-3) * t11967 + t11972 + F::new(0.21722835846488666732e-1) * t11973 * t375;
    t11976
}
