//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1104/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1104(t39939: f64, t11842: f64, t1584: f64, t10868: f64, t7628: f64, t7629: f64, t2096: f64, t2665: f64, t565: f64, t10711: f64, t11696: f64, t37936: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39940 = 0.10975748638225852664e-1_f64 * t39939;
    let t39941 = t1584 * t11842;
    let t39942 = 0.23115257973478049502e0_f64 * t39941;
    let t39958 = t7628 * t10868 * t7629;
    let t39960 = t2665 * t2096;
    let t39961 = t565 * t39960;
    let t39962 = t39961 * t10711;
    let t39963 = 0.14282990759302185292e-1_f64 * t39962;
    let t39964 = t37936 * t11696;
    (t39940, t39942, t39958, t39960, t39961, t39963, t39964)
}
