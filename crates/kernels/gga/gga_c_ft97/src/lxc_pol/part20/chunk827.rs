//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 827/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk827<F: Float>(t1501: F, t2801: F, t2843: F, t296: F, t1901: F, t24937: F, t24941: F, t24946: F, t24951: F, t24955: F, t24957: F, t24960: F, t24962: F, t25185: F, t25190: F, t25194: F, t25195: F, t25198: F, t25202: F, t25206: F, t25210: F, t446: F) -> (F, F, F, F) {
    let t25213 = t1501 * t2801;
    let t25214 = t2843 * t25213;
    let t25215 = t296 * t25214;
    let t25218 = -4.0 / 9.0 * t1901 * t24937 - 2.0 / 9.0 * t1901 * t24941 + 4.0 / 3.0 * t446 * t24946 - 2.0 * t446 * t24951 - t24955 - t446 * t24957 / 3.0 + 2.0 / 9.0 * t24960 + 2.0 / 9.0 * t24962 - t446 * t25185 / 3.0 + 2.0 / 3.0 * t446 * t25190 - t25194 + 2.0 / 9.0 * t25195 + 2.0 / 3.0 * t446 * t25198 - t446 * t25202 / 3.0 - 2.0 / 3.0 * t446 * t25206 - 2.0 / 9.0 * t446 * t25210 + 2.0 / 3.0 * t446 * t25215;
    (t25213, t25214, t25215, t25218)
}
