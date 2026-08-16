//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1110/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1110(t11842: f64, t1584: f64, t37985: f64, t38003: f64, t10868: f64, t7628: f64, t7629: f64, t2096: f64, t2665: f64, t565: f64, t10711: f64, t11696: f64, t37936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39941 = t1584 * t11842;
    let t39943 = 0.11902492299418487743e0_f64 * t37985;
    let t39950 = 0.32524801797942610062e-3_f64 * t38003;
    let t39958 = t7628 * t10868 * t7629;
    let t39960 = t2665 * t2096;
    let t39961 = t565 * t39960;
    let t39962 = t39961 * t10711;
    let t39964 = t37936 * t11696;
    (t39941, t39943, t39950, t39958, t39960, t39961, t39962, t39964)
}
