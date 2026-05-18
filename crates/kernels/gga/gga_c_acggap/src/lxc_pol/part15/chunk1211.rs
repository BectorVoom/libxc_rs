//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1211/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1211<F: Float>(t34043: F, t34054: F, t34056: F, t34058: F, t34059: F, t36918: F, t38929: F, t38934: F, t38937: F, t38939: F, t38942: F, t38946: F, t38950: F, t38954: F, t38958: F, t38960: F, t38964: F, t38968: F) -> F {
    let t41421 = F::new(0.76220476654346199062e-2) * t34043 - F::new(0.7145669686344956162e-3) * t38929 - t36918 - F::new(0.52832795046534975476e-1) * t34054 - F::new(0.42874018118069736972e-3) * t38934 - F::new(0.28582678745379824648e-2) * t34056 - t34058 + F::new(0.68598428988911579155e-1) * t38937 + F::new(0.37737710747524982483e-2) * t38939 + t38942 / F::new(16.0) + F::new(0.25158473831683321655e-2) * t34059 + F::new(0.42874018118069736972e-2) * t38946 + F::new(0.42874018118069736972e-2) * t38950 + F::new(0.42874018118069736972e-2) * t38954 + F::new(0.28582678745379824648e-2) * t38958 - F::new(0.94344276868812456206e-2) * t38960 - F::new(0.94344276868812456206e-2) * t38964 - F::new(0.94344276868812456206e-2) * t38968;
    t41421
}
