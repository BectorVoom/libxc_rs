//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1310/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1310(t100034: f64, t100930: f64, t100933: f64, t100936: f64, t100940: f64, t100942: f64, t100945: f64, t100950: f64, t100952: f64, t100954: f64, t100957: f64, t101612: f64, t11223: f64, t15109: f64, t1872: f64, t20811: f64, t28265: f64, t28295: f64, t29087: f64, t3669: f64, t5394: f64, t67159: f64, t7809: f64, t7812: f64, t8117: f64) -> f64 {
    let t101615 = 4.0_f64 * t1872 * t28295 * t3669 + 4.0_f64 * t11223 * t29087 - 2.0_f64 * t15109 * t8117 - t20811 * t7809 - 2.0_f64 * t28265 * t5394 + 2.0_f64 * t67159 * t7812 + t100034 + t100930 + t100933 + t100936 + t100940 - t100942 - t100945 + t100950 + t100952 - t100954 + t100957 - t101612;
    t101615
}
