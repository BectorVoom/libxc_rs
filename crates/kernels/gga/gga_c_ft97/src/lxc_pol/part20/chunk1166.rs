//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1166/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1166<F: Float>(t6867: F, t8232: F, t28369: F, t8392: F, t28375: F, t28150: F, t1882: F, t28120: F, t28257: F, t28319: F, t28392: F, t10153: F, t1175: F, t14245: F, t1901: F, t2405: F, t2413: F, t24395: F, t2606: F, t2619: F, t28345: F, t28355: F, t3891: F, t446: F, t53642: F, t6154: F, t6837: F, t6921: F, t729: F) -> (F,) {
    let t111215 = t8232 * t6867;
    let t111221 = 4.0 / 9.0 * t8392 * t28369;
    let t111223 = 4.0 / 27.0 * t8392 * t28375;
    let t111225 = 2.0 / 27.0 * t8392 * t28150;
    let t111227 = 2.0 / 9.0 * t1882 * t28120;
    let t111237 = 2.0 / 9.0 * t1882 * t28257;
    let t111239 = 2.0 / 9.0 * t1882 * t28319;
    let t111241 = 2.0 / 9.0 * t1882 * t28392;
    let t111242 = -t446 * t729 * t1175 * t24395 / 3.0 - t446 * t729 * t2619 * t6837 / 3.0 + t446 * t729 * t10153 * t6921 / 3.0 + 2.0 / 3.0 * t446 * t729 * t6154 * t14245 - 4.0 / 27.0 * t111215 + 4.0 / 27.0 * t1901 * t53642 * t28345 + t111221 + t111223 - t111225 + t111227 + t1901 * t2606 * t28355 * t2413 / 9.0 + 2.0 / 27.0 * t1901 * t3891 * t28355 * t2405 - t111237 + t111239 + t111241;
    (t111242,)
}
