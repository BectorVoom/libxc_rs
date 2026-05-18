//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 531/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk531<F: Float>(t1035: F, t3065: F, t3029: F, t3032: F, t3037: F, t3040: F, t3047: F, t3049: F, t3051: F, t3054: F, t3058: F, t3062: F) -> (F, F) {
    let t3066 = t1035 * t3065;
    let t3068 = F::new(0.72463633678258804342e-6) * t3029 + F::new(0.61789714048124642274e-4) * t3032 - F::new(0.12872857093359300474e-5) * t3037 + F::new(0.11594181388521408695e-4) * t3040 + F::new(0.10567613244746075633e-6) * t3047 - F::new(0.2318836277704281739e-4) * t3049 + F::new(0.2318836277704281739e-4) * t3051 + F::new(0.19323635647535681159e-6) * t3054 - F::new(0.343574241813184411e-6) * t3058 - F::new(0.84412963981222021454e-7) * t3062 - F::new(0.84412963981222021454e-7) * t3066;
    (t3066, t3068)
}
