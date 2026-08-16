//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1200/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1200<F: Float>(t5351: F, t7948: F, t309: F, t556: F, t322: F, t29979: F, t620: F, t119: F, t150: F, t157: F, t159: F, t187: F, t2143: F, t2146: F, t2152: F, t2331: F, t29997: F, t32087: F, t32091: F, t32093: F, t32096: F, t32109: F, t32121: F, t33727: F, t33818: F, t36400: F, t464: F, t616: F, t619: F, t7931: F, t9025: F, t9034: F, t9044: F, t929: F) -> F {
    let t36405 = t7948 * t5351;
    let t36416 = t556 * t309;
    let t36417 = t36416 * t322;
    let t36419 = t29979 * t620 * t36417;
    let t36425 = -F::cast_from(0.65854491829355115987e0_f64) * t32087 - F::cast_from(0.8673628188205199462e0_f64) * t2143 * t9044 - F::cast_from(0.17347256376410398924e1_f64) * t33727 * t9034 - t32091 + F::cast_from(0.17347256376410398924e1_f64) * t32093 + F::cast_from(0.17347256376410398924e1_f64) * t32096 - F::cast_from(0.13170898365871023197e1_f64) * t33818 * t464 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t36400 * t150 * t187 - F::cast_from(0.13170898365871023197e1_f64) * t36405 - t32109 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2152 * t2331 * t929 * t157 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t29997 * t9025 + F::cast_from(0.26341796731742046394e1_f64) * t32121 - F::cast_from(0.69389025505641595696e1_f64) * t36419 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t619 * t159 * t36400;
    t36425
}
