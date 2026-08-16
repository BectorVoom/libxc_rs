//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 628/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk628(t772: f64, t5508: f64, t5509: f64, t1586: f64, t5432: f64, t2021: f64, t2005: f64, t2013: f64, t2016: f64, t2025: f64, t5465: f64, t5468: f64, t5471: f64, t5479: f64, t5481: f64, t5484: f64, t5488: f64, t5494: f64, t5499: f64, t5503: f64, t782: f64, t788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t783 = 0.0_f64 < t772;
    let t5510 = t5508 * t5509;
    let t5511 = t1586 * t5510;
    let t5515 = piecewise3(t783, t5432, -t5432);
    let t5516 = t2021 * t5515;
    let t5517 = t1586 * t5516;
    let t5520 = 0.2698618307426597582e-1_f64 * t5465 * t788 + 0.17990788716177317213e-1_f64 * t5468 + 0.17990788716177317213e-1_f64 * t5471 * t2016 - 0.5397236614853195164e-1_f64 * t2005 * t2025 - t5479 + 0.59969295720591057378e-2_f64 * t5481 - 0.17990788716177317213e-1_f64 * t5484 + 0.11993859144118211476e-1_f64 * t2013 * t5488 - 0.17990788716177317213e-1_f64 * t2013 * t5494 - 0.17990788716177317213e-1_f64 * t2013 * t5499 + 0.89953943580886586067e-2_f64 * t2013 * t5503 + 0.5397236614853195164e-1_f64 * t782 * t5511 - 0.2698618307426597582e-1_f64 * t782 * t5517;
    (t5510, t5511, t5515, t5516, t5517, t5520)
}
