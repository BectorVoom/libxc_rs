//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1142/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1142(t11970: f64, t367: f64, t1020: f64, t3230: f64, t11924: f64, t11927: f64, t11930: f64, t11933: f64, t11938: f64, t11941: f64, t11944: f64, t11947: f64, t11952: f64, t11954: f64, t11956: f64, t11962: f64, t11965: f64, t11967: f64, t3120: f64, t3208: f64, t375: f64) -> (f64, f64) {
    let t11972 = 0.63517063878621832551e-4_f64 * t367 * t11970;
    let t11973 = t1020 * t3230;
    let t11976 = -0.85748036236139473944e-3_f64 * t11924 + 0.12862205435420921092e-2_f64 * t11927 * t11930 + 0.68598428988911579154e-2_f64 * t11933 * t3120 + 0.85748036236139473944e-3_f64 * t11938 - 0.12862205435420921092e-2_f64 * t11941 * t11944 - 0.68598428988911579154e-2_f64 * t11947 * t3208 - 0.42874018118069736972e-3_f64 * t11952 - 0.45732285992607719436e-2_f64 * t11954 - 0.14291339372689912324e-3_f64 * t11956 - 0.53100265402527852012e-1_f64 * t11962 * t375 + 0.14481890564325777821e-1_f64 * t11965 + 0.7622047665434619906e-3_f64 * t11967 + t11972 + 0.21722835846488666732e-1_f64 * t11973 * t375;
    (t11973, t11976)
}
