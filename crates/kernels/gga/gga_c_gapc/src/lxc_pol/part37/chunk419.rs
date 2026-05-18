//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 419/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk419<F: Float>(t2188: F, t297: F, t314: F, t129: F, t519: F, t919: F, t1179: F, t2042: F, t2043: F, t2046: F, t2134: F, t2155: F, t2160: F, t2161: F, t2165: F, t2166: F, t284: F, t316: F, t731: F, t763: F, t821: F) -> (F, F) {
    let t2189 = t2188 * t297;
    let t2190 = t2189 * t314;
    let t2191 = t129 * t2190;
    let t2196 = t519 * t919;
    let t2199 = t2042 + F::new(0.1252584660908875509e-2) * t2043 * t316 - F::new(0.93943849568165663176e-3) * t2046 * t316 - F::new(0.93943849568165663176e-3) * t731 * t821 + F::new(0.28183154870449698953e-3) * t2155 * t316 - F::new(0.11135477635479903275e-5) * t2160 * t2161 + F::new(0.4871771465522457683e-5) * t2165 * t2166 + F::new(0.28183154870449698953e-3) * t284 * t2191 + F::new(0.56366309740899397906e-3) * t763 * t821 - t1179 + t2134 - F::new(0.2740028945738165176e-5) * t2165 * t2196;
    (t2190, t2199)
}
