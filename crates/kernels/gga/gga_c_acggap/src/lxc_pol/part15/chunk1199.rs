//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1199/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1199<F: Float>(t33799: F, t9171: F, t32130: F, t38052: F, t9029: F, t40733: F, t7963: F, t8306: F, t2143: F, t2146: F, t2147: F, t2222: F, t33107: F, t33118: F, t33120: F, t38181: F, t38185: F, t38187: F, t38190: F, t38194: F, t557: F, t633: F, t6557: F, t6569: F, t9995: F) -> F {
    let t41176 = t33799 * t9171;
    let t41187 = t32130 * t38052 * t9029;
    let t41192 = t7963 * t8306 * t40733;
    let t41194 = -t33107 - F::new(0.13877805101128319139e2) * t38181 + F::new(0.13170898365871023197e1) * t2222 * t6569 - F::new(0.4336814094102599731e0) * t2143 * t9995 - F::new(0.17347256376410398924e1) * t41176 + F::new(0.69389025505641595696e1) * t38185 + F::new(0.34694512752820797848e1) * t38190 - t38194 - F::new(0.26020884564615598386e1) * t33118 + F::new(0.8673628188205199462e0) * t2146 * t2147 * t633 * t6557 - F::new(0.26020884564615598386e1) * t33120 - F::new(0.34694512752820797848e1) * t41187 - F::new(0.13170898365871023197e1) * t38187 * t557 + F::new(0.8673628188205199462e0) * t41192;
    t41194
}
