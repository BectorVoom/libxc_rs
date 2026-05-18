//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1110/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1110<F: Float>(t11842: F, t1584: F, t37985: F, t38003: F, t10868: F, t7628: F, t7629: F, t2096: F, t2665: F, t565: F, t10711: F, t11696: F, t37936: F) -> (F, F, F, F, F, F, F, F) {
    let t39941 = t1584 * t11842;
    let t39943 = F::new(0.11902492299418487743e0) * t37985;
    let t39950 = F::new(0.32524801797942610062e-3) * t38003;
    let t39958 = t7628 * t10868 * t7629;
    let t39960 = t2665 * t2096;
    let t39961 = t565 * t39960;
    let t39962 = t39961 * t10711;
    let t39964 = t37936 * t11696;
    (t39941, t39943, t39950, t39958, t39960, t39961, t39962, t39964)
}
