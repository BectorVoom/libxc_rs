//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 801/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk801<F: Float>(t11764: F, t11767: F, t11770: F, t11772: F, t11778: F, t11782: F, t11784: F, t11787: F, t11790: F, t11792: F, t11796: F, t11800: F, t11804: F, t11810: F, t11813: F, t11925: F, t11930: F, t11932: F) -> F {
    let t12396 = -F::new(0.375e0) * t11764 - F::cast_from(0.62499999999999999999e-1_f64) * t11767 - F::new(0.60703125e-1) * t11770 - F::new(0.625e-1) * t11772 - F::new(0.5625e0) * t11778 - F::cast_from(0.13489583333333333333e-1_f64) * t11782 + F::cast_from(0.303515625e-1_f64) * t11784 + F::cast_from(0.40468749999999999999e-1_f64) * t11787 + F::cast_from(0.13489583333333333333e-1_f64) * t11790 - F::new(0.28125e0) * t11792 + F::cast_from(0.27777777777777777777e-1_f64) * t11796 - F::new(0.28125e0) * t11800 - F::new(0.9375e-1) * t11804 + F::cast_from(0.29976851851851851851e-2_f64) * t11810 - F::cast_from(0.13489583333333333333e-1_f64) * t11813 + F::new(0.9375e-1) * t11925 + F::new(0.60703125e-1) * t11930 + F::new(0.1875e0) * t11932;
    t12396
}
