//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 628/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk628<F: Float>(t772: F, t5508: F, t5509: F, t1586: F, t5432: F, t2021: F, t2005: F, t2013: F, t2016: F, t2025: F, t5465: F, t5468: F, t5471: F, t5479: F, t5481: F, t5484: F, t5488: F, t5494: F, t5499: F, t5503: F, t782: F, t788: F) -> (F, F, F, F, F, F) {
    let t783 = F::new(0.0) < t772;
    let t5510 = t5508 * t5509;
    let t5511 = t1586 * t5510;
    let t5515 = piecewise3::<f64>(t783, t5432, -t5432);
    let t5516 = t2021 * t5515;
    let t5517 = t1586 * t5516;
    let t5520 = F::new(0.2698618307426597582e-1) * t5465 * t788 + F::new(0.17990788716177317213e-1) * t5468 + F::new(0.17990788716177317213e-1) * t5471 * t2016 - F::new(0.5397236614853195164e-1) * t2005 * t2025 - t5479 + F::new(0.59969295720591057378e-2) * t5481 - F::new(0.17990788716177317213e-1) * t5484 + F::new(0.11993859144118211476e-1) * t2013 * t5488 - F::new(0.17990788716177317213e-1) * t2013 * t5494 - F::new(0.17990788716177317213e-1) * t2013 * t5499 + F::new(0.89953943580886586067e-2) * t2013 * t5503 + F::new(0.5397236614853195164e-1) * t782 * t5511 - F::new(0.2698618307426597582e-1) * t782 * t5517;
    (t5510, t5511, t5515, t5516, t5517, t5520)
}
