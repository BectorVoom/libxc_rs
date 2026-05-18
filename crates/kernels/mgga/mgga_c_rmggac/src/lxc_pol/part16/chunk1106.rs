//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1106/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1106<F: Float>(t10459: F, t333: F, t1356: F, t1668: F, t43722: F, t43745: F, t43746: F, t43974: F, t44405: F, t47306: F, t47310: F, t47316: F, t47321: F, t47325: F, t47327: F, t47331: F, t47333: F, t47335: F, t47340: F, t530: F, t5888: F, t884: F, t9639: F) -> (F, F) {
    let t48976 = t10459 * t333;
    let t48990 = F::new(0.85129199786595678799e-5) * t47306 - F::new(0.77813409179935112652e-4) * t47310 - t43722 + F::new(0.2993560425465952141e-1) * t47316 - F::new(0.23948483403727617128e0) * t1356 * t43974 * t5888 + F::new(0.59871208509319042821e-1) * t884 * t48976 - F::new(0.5107751987195740728e-4) * t47321 - F::new(0.3405167991463827152e-4) * t47325 + F::new(0.1702583995731913576e-4) * t47327 + F::new(0.1702583995731913576e-4) * t47331 + F::new(0.212822999466489197e-4) * t47333 + F::new(0.212822999466489197e-4) * t47335 - F::new(0.4726e1) * t530 * t44405 - F::new(0.4726e1) * t1668 * t9639 + t43745 + t43746 + F::new(0.17961362552795712846e0) * t47340;
    (t48976, t48990)
}
