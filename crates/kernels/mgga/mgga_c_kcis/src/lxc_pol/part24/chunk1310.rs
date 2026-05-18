//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1310/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1310<F: Float>(t100034: F, t100930: F, t100933: F, t100936: F, t100940: F, t100942: F, t100945: F, t100950: F, t100952: F, t100954: F, t100957: F, t101612: F, t11223: F, t15109: F, t1872: F, t20811: F, t28265: F, t28295: F, t29087: F, t3669: F, t5394: F, t67159: F, t7809: F, t7812: F, t8117: F) -> F {
    let t101615 = F::new(4.0) * t1872 * t28295 * t3669 + F::new(4.0) * t11223 * t29087 - F::new(2.0) * t15109 * t8117 - t20811 * t7809 - F::new(2.0) * t28265 * t5394 + F::new(2.0) * t67159 * t7812 + t100034 + t100930 + t100933 + t100936 + t100940 - t100942 - t100945 + t100950 + t100952 - t100954 + t100957 - t101612;
    t101615
}
