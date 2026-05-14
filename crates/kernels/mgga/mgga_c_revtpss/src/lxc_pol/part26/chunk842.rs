//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 842/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk842<F: Float>(t1021: F, t3201: F, t362: F, t40: F, t361: F, t351: F, t1058: F, t3231: F, t1054: F, t2434: F, t371: F, t373: F, t367: F, t1020: F, t3230: F, t11924: F, t11927: F, t11930: F, t11933: F, t11938: F, t11941: F, t11944: F, t11947: F, t11952: F, t11954: F, t3120: F, t3208: F, t375: F) -> (F,) {
    let t11956 = t1021 * t3201;
    let t11958 = t362 * t362;
    let t11960 = 1.0 / t40 / t11958;
    let t11961 = t361 * t11960;
    let t11962 = t351 * t11961;
    let t11965 = t3231 * t1058;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4 * t367 * t11970;
    let t11973 = t1020 * t3230;
    let t11976 = -0.85748036236139473944e-3 * t11924 + 0.12862205435420921092e-2 * t11927 * t11930 + 0.68598428988911579154e-2 * t11933 * t3120 + 0.85748036236139473944e-3 * t11938 - 0.12862205435420921092e-2 * t11941 * t11944 - 0.68598428988911579154e-2 * t11947 * t3208 - 0.42874018118069736972e-3 * t11952 - 0.45732285992607719436e-2 * t11954 - 0.14291339372689912324e-3 * t11956 - 0.53100265402527852012e-1 * t11962 * t375 + 0.14481890564325777821e-1 * t11965 + 0.7622047665434619906e-3 * t11967 + t11972 + 0.21722835846488666732e-1 * t11973 * t375;
    (t11976,)
}
