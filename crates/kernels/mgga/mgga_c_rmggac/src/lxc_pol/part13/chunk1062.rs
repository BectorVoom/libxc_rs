//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1062/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1062<F: Float>(t39840: F, t39842: F, t333: F, t9565: F, t35407: F, t35413: F, t35424: F, t39813: F, t39818: F, t39830: F, t39833: F, t39838: F, t39855: F, t39859: F, t39861: F, t39864: F, t39869: F, t4041: F, t884: F, t9405: F) -> (F, F) {
    let t43157 = F::new(0.49658699875514145965e-4) * t39840;
    let t43158 = F::new(0.11918087970123395032e-3) * t39842;
    let t43163 = t9565 * t333;
    let t43167 = -F::new(0.23948483403727617128e0) * t4041 * t9405 + F::new(0.15323255961587222184e-3) * t39813 + F::new(0.5107751987195740728e-4) * t39818 - F::new(0.95793933614910468511e0) * t35407 - F::new(0.3193131120497015617e0) * t35413 - F::new(0.36366215538993788974e0) * t35424 - F::new(0.85129199786595678799e-5) * t39830 - F::new(0.1702583995731913576e-4) * t39833 - F::new(0.23942587439980034662e-4) * t39838 - t43157 + t43158 - F::new(0.3405167991463827152e-4) * t39855 + F::new(0.3405167991463827152e-4) * t39859 + F::new(0.5107751987195740728e-4) * t39861 + F::new(0.68186654135613354325e-2) * t39864 + F::new(0.11974241701863808564e0) * t884 * t43163 - F::new(0.212822999466489197e-4) * t39869;
    (t43163, t43167)
}
