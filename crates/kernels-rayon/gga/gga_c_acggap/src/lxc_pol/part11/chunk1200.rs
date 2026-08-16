//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1200/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1200(t5351: f64, t7948: f64, t309: f64, t556: f64, t322: f64, t29979: f64, t620: f64, t119: f64, t150: f64, t157: f64, t159: f64, t187: f64, t2143: f64, t2146: f64, t2152: f64, t2331: f64, t29997: f64, t32087: f64, t32091: f64, t32093: f64, t32096: f64, t32109: f64, t32121: f64, t33727: f64, t33818: f64, t36400: f64, t464: f64, t616: f64, t619: f64, t7931: f64, t9025: f64, t9034: f64, t9044: f64, t929: f64) -> f64 {
    let t36405 = t7948 * t5351;
    let t36416 = t556 * t309;
    let t36417 = t36416 * t322;
    let t36419 = t29979 * t620 * t36417;
    let t36425 = -0.65854491829355115987e0_f64 * t32087 - 0.8673628188205199462e0_f64 * t2143 * t9044 - 0.17347256376410398924e1_f64 * t33727 * t9034 - t32091 + 0.17347256376410398924e1_f64 * t32093 + 0.17347256376410398924e1_f64 * t32096 - 0.13170898365871023197e1_f64 * t33818 * t464 + 0.65854491829355115987e0_f64 * t119 * t36400 * t150 * t187 - 0.13170898365871023197e1_f64 * t36405 - t32109 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t2331 * t929 * t157 - 0.17347256376410398924e1_f64 * t7931 * t29997 * t9025 + 0.26341796731742046394e1_f64 * t32121 - 0.69389025505641595696e1_f64 * t36419 - 0.4336814094102599731e0_f64 * t616 * t619 * t159 * t36400;
    t36425
}
